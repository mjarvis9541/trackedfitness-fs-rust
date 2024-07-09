use leptos::*;
use leptos_router::*;

use chrono::NaiveDate;

use crate::component::button::Button;
use crate::component::icon::IconCopy;
use crate::util::param::{get_date, get_username};

#[cfg(feature = "ssr")]
use {
    crate::{
        auth::model::User, auth::service::get_request_user, diet::model::Diet, error::Error,
        setup::get_pool,
    },
    chrono::Days,
};

#[server(endpoint = "diet-copy-previous-day")]
pub async fn diet_copy_previous_day(
    username: String,
    date: NaiveDate,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    let target_user = User::get_by_username(&pool, &username)
        .await?
        .ok_or(Error::NotFound)?;
    Diet::can_create(&user, target_user.id)?;
    let previous_date = date.checked_sub_days(Days::new(1)).expect("valid date");
    let previous_date_diet_logs = Diet::all_by_user_id_date(&pool, user.id, previous_date).await?;
    if previous_date_diet_logs.is_empty() {
        return Err(ServerFnError::new("Nothing to add"));
    }
    Diet::bulk_create_from_previous_day(&pool, user.id, &date, &previous_date_diet_logs, user.id)
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

    view! {
        <div>
            <ActionForm action class="contents">
                <input type="hidden" name="username" value=username/>
                <input type="hidden" name="date" value=date/>
                <Button label="Copy All Yesterday">
                    <IconCopy/>
                </Button>
            </ActionForm>
        </div>
    }
}
