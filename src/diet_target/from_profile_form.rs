use chrono::prelude::*;
use leptos::*;
use leptos_router::*;

use crate::component::button::Button;
use crate::component::icon::IconEditC;
use crate::util::param::{get_date, get_username};
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{
    auth::model::User,
    auth::service::get_request_user,
    diet_target::model::DietTargetInput,
    diet_target::model::{DietTarget, DietTargetBase},
    error::Error,
    profile::fitness_goal::{FitnessGoal, TargetModifier},
    profile::model::Profile,
    setup::get_pool,
};

#[server]
pub async fn target_from_profile_create(
    username: String,
    date: NaiveDate,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let target_user = User::get_by_username(&pool, &username)
        .await?
        .ok_or(Error::NotFound)?;

    DietTarget::can_create(&user, target_user.id).await?;
    DietTarget::validate_date(date)?;

    let profile = Profile::get_latest_by_username(&pool, &user.username, date)
        .await?
        .ok_or(Error::NotFound)?;

    let Some(latest_weight) = profile.latest_weight else {
        return Err(ServerFnError::new(
            "Weight log needs to be logged in progress to proceed.",
        ));
    };

    let fitness_goal = profile
        .fitness_goal
        .parse::<FitnessGoal>()
        .map_err(|_| Error::InternalServer)?;
    let modifier: TargetModifier = fitness_goal.into();
    let tdee = profile.get_total_daily_energy_expenditure();

    let database_input =
        DietTargetInput::calculate_nutrients(modifier, tdee, target_user.id, date, latest_weight);

    DietTargetBase::create(&pool, database_input, user.id).await?;
    Ok(())
}

#[component]
pub fn TargetFromProfileCreateForm(
    action: Action<TargetFromProfileCreate, Result<(), ServerFnError>>,
) -> impl IntoView {
    let params = use_params_map();
    let username = move || get_username(&params);
    let date = move || get_date(&params).to_string();

    let _action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || extract_other_errors(action_value, &["name"]);
    let non_field_errors = move || get_non_field_errors(action_value);

    view! {
        <div class="mb-4 text-red-500 font-bold">{action_error}</div>
        <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>
        <ActionForm action class="contents">
            <input type="hidden" name="username" value=username/>
            <input type="hidden" name="date" value=date/>
            <Button label="New Diet Target (Auto)">
                <IconEditC/>
            </Button>
        </ActionForm>
    }
}
