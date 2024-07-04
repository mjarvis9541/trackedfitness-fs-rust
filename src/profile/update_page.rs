use leptos::*;
use leptos_router::*;

use chrono::prelude::*;
use rust_decimal::Decimal;

use crate::component::button::SubmitButton;
use crate::component::input::{NumberInput, TextInputImproved};
use crate::component::select::{
    FieldSelect, ACTIVITY_LEVEL_OPTIONS, FITNESS_GOAL_OPTIONS, SEX_OPTIONS,
};
use crate::component::template::{DetailPageTemplate, ErrorComponent, LoadingComponent};
use crate::profile::model::Profile;
use crate::util::param::get_username;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{
    auth::model::User, auth::service::get_request_user, error::Error, profile::model::ProfileBase,
    setup::get_pool,
};

#[server(endpoint = "get-profile-update")]
async fn get_profile_update(username: String) -> Result<Profile, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    User::check_view_permission(&pool, &user, &username).await?;
    let date = Utc::now().date_naive();
    let object = Profile::get_latest_by_username(&pool, &username, date)
        .await?
        .ok_or(Error::NotFound)?;

    Ok(object)
}

#[server]
async fn profile_update(
    username: String,
    sex: String,
    height: Decimal,
    date_of_birth: NaiveDate,
    activity_level: String,
    fitness_goal: String,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    User::check_view_permission(&pool, &user, &username).await?;

    let date = Utc::now().date_naive();
    let object = Profile::get_latest_by_username(&pool, &username, date)
        .await?
        .ok_or(Error::NotFound)?;

    object.can_update(&user).await?;

    Profile::validate(
        &sex,
        &activity_level,
        &fitness_goal,
        height,
        Decimal::from(50),
        date_of_birth,
    )?;
    ProfileBase::update(
        &pool,
        object.id,
        &sex,
        height,
        date_of_birth,
        &activity_level,
        &fitness_goal,
        user.id,
    )
    .await?;

    leptos_axum::redirect(&format!("/users/{username}/profile"));
    Ok(())
}

#[component]
pub fn ProfileUpdatePage() -> impl IntoView {
    let params = use_params_map();
    let username = move || get_username(&params);

    let action = Action::<ProfileUpdate, _>::server();
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || extract_other_errors(action_value, &["name", "ordering"]);
    let non_field_errors = move || get_non_field_errors(action_value);

    let resource = Resource::new(username, get_profile_update);
    let response = move || {
        resource.and_then(|data| {
            let data = data.clone();
            view! {
                <ActionForm action>
                    <input type="hidden" name="id" value=data.id.to_string()/>
                    <input type="hidden" name="username" value=data.username/>
                    <FieldSelect
                        name="activity_level"
                        options=&ACTIVITY_LEVEL_OPTIONS
                        value=data.activity_level
                    />
                    <FieldSelect
                        name="fitness_goal"
                        options=&FITNESS_GOAL_OPTIONS
                        value=data.fitness_goal
                    />
                    <FieldSelect name="sex" options=&SEX_OPTIONS value=data.sex/>

                    <NumberInput
                        action_value
                        name="height"
                        label="Height (cm)"
                        step="1"
                        placeholder="Enter your height in cm"
                        value=data.height.to_string()
                    />
                    <TextInputImproved
                        action_value
                        name="date_of_birth"
                        label="Date of Birth"
                        input_type="date"
                        value=data.date_of_birth.to_string()
                    />
                    <SubmitButton loading=action_loading label="Update Profile"/>
                </ActionForm>
            }
        })
    };

    view! {
        <DetailPageTemplate title="Edit Profile">
            <div class="mb-4 text-red-500 font-bold">{action_error}</div>
            <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>
            <Transition fallback=LoadingComponent>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors/> }
                }>{response}</ErrorBoundary>
            </Transition>
        </DetailPageTemplate>
    }
}