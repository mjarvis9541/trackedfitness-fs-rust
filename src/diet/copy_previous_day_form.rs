use leptos::*;
use leptos_router::*;

use chrono::prelude::*;

use crate::component::button::Button;
use crate::component::icon::IconCopy;
use crate::error_extract::extract_error_message;
use crate::util::param::{get_date, get_username};

#[cfg(feature = "ssr")]
use crate::{auth::model::User, auth::service::get_request_user, diet::model::Diet, error::Error};

#[server(endpoint = "diet-copy-previous-day")]
pub async fn diet_copy_previous_day(
    username: String,
    date: NaiveDate,
) -> Result<(), ServerFnError> {
    let request_user = get_request_user()?;
    let pool = expect_context::<sqlx::PgPool>();

    let user = User::get_by_username(&pool, &username)
        .await?
        .ok_or(Error::NotFound)?;

    Diet::can_create(&request_user, user.id)?;

    let previous_date = date - chrono::TimeDelta::days(1);

    let previous_day_diet_logs = Diet::all_by_user_id_date(&pool, user.id, previous_date).await?;
    if previous_day_diet_logs.is_empty() {
        return Err(ServerFnError::new("Nothing to add"));
    }
    Diet::bulk_create_from_previous_day(
        &pool,
        user.id,
        &date,
        &previous_day_diet_logs,
        request_user.id,
    )
    .await?;
    Ok(())
}

#[component]
pub fn DietCopyPreviousDayForm(
    action: Action<DietCopyPreviousDay, Result<(), ServerFnError>>,
) -> impl IntoView {
    let params = use_params_map();
    let username = move || get_username(&params);
    let date = move || get_date(&params).to_string();

    let error = move || extract_error_message(&action);
    view! {
        {error}
        <ActionForm action class="contents">
            <input type="hidden" name="username" value=username/>
            <input type="hidden" name="date" value=date/>
            <Button label="Copy All Yesterday">
                <IconCopy/>
            </Button>
        </ActionForm>
    }
}
