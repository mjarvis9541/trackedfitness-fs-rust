use leptos::*;
use leptos_router::*;

use chrono::prelude::*;
use rust_decimal::Decimal;

use crate::component::button::SubmitButton;
use crate::component::input::{NumberInput, TextInput};
use crate::component::select::FieldSelect;
use crate::component::template::DetailPageTemplate;
use crate::util::param::get_username;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

use super::activity_level::ActivityLevel;
use super::fitness_goal::FitnessGoal;
use super::sex::Sex;

#[cfg(feature = "ssr")]
use crate::{
    auth::model::User,
    auth::service::get_request_user,
    error::Error,
    profile::model::{Profile, ProfileQuery},
    progress::model::Progress,
    setup::get_pool,
};

#[server(endpoint = "profile-create")]
pub async fn profile_create(
    username: String,
    fitness_goal: String,
    activity_level: String,
    sex: String,
    height: Decimal,
    weight: Decimal,
    date_of_birth: NaiveDate,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let target_user = User::get_by_username(&pool, &username)
        .await?
        .ok_or(Error::NotFound)?;

    Profile::can_create(&user, target_user.id).await?;
    ProfileQuery::validate(
        &sex,
        &activity_level,
        &fitness_goal,
        height,
        weight,
        date_of_birth,
    )?;

    let latest_weight = Progress::get_latest_weight(&pool, user.id).await?;
    dbg!(&latest_weight);
    if latest_weight.is_none() {
        let date = Utc::now().date_naive();
        Progress::create(
            &pool,
            target_user.id,
            date,
            Some(weight),
            None,
            None,
            user.id,
        )
        .await?;
    }

    Profile::create(
        &pool,
        target_user.id,
        &sex,
        height,
        date_of_birth,
        &activity_level,
        &fitness_goal,
        user.id,
    )
    .await?;

    leptos_axum::redirect(&format!("/users/{}", target_user.username));
    Ok(())
}

#[component]
pub fn ProfileCreatePage() -> impl IntoView {
    let params = use_params_map();
    let username = move || get_username(&params);

    let action = Action::<ProfileCreate, _>::server();
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || extract_other_errors(action_value, &["name", "ordering"]);
    let non_field_errors = move || get_non_field_errors(action_value);

    let goal_options = FitnessGoal::to_form_options();
    let activity_options = ActivityLevel::to_form_options();
    let sex_options = Sex::to_form_options();

    view! {
        <DetailPageTemplate title="Set up Profile">
            <div class="mb-4 text-red-500 font-bold">{action_error}</div>
            <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>
            <ActionForm action>
                <input type="hidden" name="username" value=username/>
                <FieldSelect name="fitness_goal" options=goal_options/>
                <FieldSelect name="activity_level" options=activity_options/>
                <FieldSelect name="sex" options=sex_options/>
                <NumberInput
                    action_value
                    name="height"
                    label="Height"
                    step="1"
                    placeholder="Enter your height in cm"
                />
                <NumberInput
                    action_value
                    name="weight"
                    label="Weight"
                    step="0.01"
                    placeholder="Enter your weight in kg"
                />
                <TextInput
                    action_value
                    name="date_of_birth"
                    label="Date of Birth"
                    input_type="date"
                />
                <SubmitButton loading=action_loading label="Create Profile"/>
            </ActionForm>
        </DetailPageTemplate>
    }
}
