use leptos::*;
use leptos_router::*;

use chrono::prelude::*;

use crate::component::button::Button;
use crate::component::icon::IconCopy;
use crate::error_extract::extract_error_message;
use crate::util::param::{get_date, get_username};

#[cfg(feature = "ssr")]
use crate::{
    auth::model::User, auth::service::get_request_user, diet::model::Diet, error::Error,
    meal_of_day::model::MealOfDay, setup::get_pool,
};

#[server(endpoint = "diet-copy-previous-meal")]
pub async fn diet_copy_previous(
    username: String,
    date: NaiveDate,
    meal_of_day_slug: String,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let target_user = User::get_by_username(&pool, &username)
        .await?
        .ok_or(Error::NotFound)?;

    Diet::can_create(&user, target_user.id)?;

    let meal_of_day = MealOfDay::get_by_slug(&pool, &meal_of_day_slug)
        .await?
        .ok_or(Error::NotFound)?;

    let previous_date = date - chrono::TimeDelta::days(1);

    let previous_date_diet_logs =
        Diet::all_by_user_id_date_meal(&pool, target_user.id, previous_date, meal_of_day.id)
            .await?;

    if previous_date_diet_logs.is_empty() {
        return Err(ServerFnError::new("Nothing to add"));
    }

    Diet::bulk_create_from_previous_day_meal(
        &pool,
        target_user.id,
        date,
        meal_of_day.id,
        &previous_date_diet_logs,
        user.id,
    )
    .await?;
    Ok(())
}

#[component]
pub fn DietCopyPreviousForm(meal: String, label: &'static str) -> impl IntoView {
    let params = use_params_map();
    let username = move || get_username(&params);
    let date = move || get_date(&params).to_string();

    let action = expect_context::<Action<DietCopyPrevious, _>>();
    let error = move || extract_error_message(&action);

    view! {
        <ActionForm action class="contents">
            <input type="hidden" name="username" value=username/>
            <input type="hidden" name="date" value=date/>
            <input type="hidden" name="meal_of_day_slug" value=meal/>
            <Button label=label>
                <IconCopy/>
            </Button>
        </ActionForm>
    }
}
