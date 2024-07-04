use leptos::*;
use leptos_router::*;

use chrono::prelude::*;
use rust_decimal::Decimal;

use crate::component::button::SubmitButton;
use crate::component::input::{NumberInput, TextInputImproved};
use crate::component::select::{
    FieldSelect, ACTIVITY_LEVEL_OPTIONS, FITNESS_GOAL_OPTIONS, SEX_OPTIONS,
};
use crate::component::template::DetailPageTemplate;
use crate::util::param::get_username;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{
    auth::model::User,
    auth::service::get_request_user,
    error::Error,
    profile::model::{Profile, ProfileBase},
    progress::model::{Progress, ProgressBase},
    setup::get_pool,
};

#[server(endpoint = "profile-create")]
pub async fn profile_create(
    username: String,
    sex: String,
    height: Decimal,
    weight: Decimal,
    date_of_birth: NaiveDate,
    activity_level: String,
    fitness_goal: String,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    let target_user = User::get_by_username(&pool, &username)
        .await?
        .ok_or(Error::NotFound)?;
    ProfileBase::can_create(&user, target_user.id).await?;
    Profile::validate(
        &sex,
        &activity_level,
        &fitness_goal,
        height,
        weight,
        date_of_birth,
    )?;

    let latest_weight = Progress::get_latest_weight(&pool, user.id).await?;

    if latest_weight.is_none() {
        let date = Utc::now().date_naive();
        ProgressBase::create(
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
    ProfileBase::create(
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

    leptos_axum::redirect(&format!("/users/{}", user.username));
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

    view! {
        <DetailPageTemplate title="Create Profile">
            <div class="mb-4 text-red-500 font-bold">{action_error}</div>
            <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>
            <ActionForm action>
                <input type="hidden" name="username" value=username/>
                <FieldSelect name="sex" options=&SEX_OPTIONS/>
                <FieldSelect name="activity_level" options=&ACTIVITY_LEVEL_OPTIONS/>
                <FieldSelect name="fitness_goal" options=&FITNESS_GOAL_OPTIONS/>

                <NumberInput
                    action_value
                    name="height"
                    label="Height (cm)"
                    step="1"
                    placeholder="Enter your height in cm"
                />
                <NumberInput
                    action_value
                    name="weight"
                    label="Weight (kg)"
                    step="0.01"
                    placeholder="Enter your weight in kg"
                />
                <TextInputImproved
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
