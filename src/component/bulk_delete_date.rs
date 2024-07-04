use std::collections::HashSet;

use leptos::*;
use leptos_router::*;

use crate::component::button::{Button, ButtonVariant};
use crate::component::icon::IconTrash;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{
    auth::model::User, auth::service::get_request_user, error::Error, setup::get_pool,
    util::server::parse_dates_from_strings,
};

#[server]
pub async fn bulk_delete_date_range(
    table: String,
    username: String,
    items: Option<HashSet<String>>,
) -> Result<u64, ServerFnError> {
    const ALLOWED_TABLES: [&str; 3] = ["food_log", "progress", "diet_target"];

    let user = get_request_user()?;
    let pool = get_pool()?;

    if !user.is_superuser && !ALLOWED_TABLES.contains(&table.as_str()) {
        return Err(ServerFnError::new(
            "You are not allowed to delete these items",
        ));
    }
    let items = items.ok_or_else(|| ServerFnError::new("Nothing selected"))?;
    let date_list = parse_dates_from_strings(&items)?;

    let target_user = User::get_by_username(&pool, &username)
        .await?
        .ok_or(Error::NotFound)?;

    if target_user.id != user.id {
        return Err(Error::Forbidden)?;
    };

    let query = sqlx::query(&format!(
        "DELETE FROM {table} WHERE user_id = $1 AND date = ANY ($2)"
    ))
    .bind(user.id)
    .bind(date_list)
    .execute(&pool)
    .await?
    .rows_affected();

    if query == 0 {
        return Err(ServerFnError::new("Nothing deleted"));
    }

    Ok(query)
}

#[component]
pub fn BulkDeleteDateRangeForm(
    table: &'static str,
    action: Action<BulkDeleteDateRange, Result<u64, ServerFnError>>,
    username: Signal<String>,
    checked_items: RwSignal<HashSet<String>>,
) -> impl IntoView {
    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        if let Ok(mut data) = BulkDeleteDateRange::from_event(&ev) {
            data.items = Some(checked_items());
            checked_items.update(|v| v.clear());
            action.dispatch(data)
        }
    };

    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || {
        extract_other_errors(
            action_value,
            &["non_field_errors", "table", "username", "items"],
        )
    };
    let non_field_errors = move || get_non_field_errors(action_value);

    view! {
        <div class="mb-4 text-red-500 font-bold">{action_error}</div>
        <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>
        <ActionForm action on:submit=handle_submit>
            <input type="hidden" name="table" value=table/>
            <input type="hidden" name="username" value=username/>
            <input type="hidden" name="items" value=""/>
            <Button
                label="Delete"
                loading=action_loading
                variant=ButtonVariant::Danger
                disabled=Signal::derive(move || checked_items.with(HashSet::is_empty))
            >
                <IconTrash/>
            </Button>
        </ActionForm>
    }
}
