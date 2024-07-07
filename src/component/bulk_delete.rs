use std::collections::HashSet;

use leptos::*;
use leptos_router::*;

use crate::component::button::{Button, ButtonVariant};
use crate::component::icon::IconTrash;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[cfg(feature = "ssr")]
use {
    crate::{
        auth::service::get_request_user, error::Error, setup::get_pool,
        util::server::parse_uuids_from_strings,
    },
    sqlx::Row,
    uuid::Uuid,
};

#[server(endpoint = "bulk-delete")]
pub async fn bulk_delete(
    table: String,
    items: Option<HashSet<String>>,
) -> Result<u64, ServerFnError> {
    const ALLOWED_TABLES: [&str; 3] = ["food_log", "progress", "diet_target"];

    let user = get_request_user()?;
    let pool = get_pool()?;

    let items = items.ok_or_else(|| ServerFnError::new("Nothing selected"))?;
    let uuid_list =
        parse_uuids_from_strings(&items).map_err(|_| ServerFnError::new("Invalid id selection"))?;

    if user.is_superuser {
        let sql = format!("DELETE FROM {table} WHERE id = ANY ($1)");
        let query = sqlx::query(&sql)
            .bind(&uuid_list)
            .execute(&pool)
            .await?
            .rows_affected();
        return Ok(query);
    }

    if !ALLOWED_TABLES.contains(&table.as_str()) {
        return Err(ServerFnError::new(
            "You are not allowed to delete these items",
        ));
    }

    let user_id = user.id;
    let check_sql = format!("SELECT user_id FROM {table} WHERE id = ANY ($1)");

    let all_owner_ids_match_user = sqlx::query(&check_sql)
        .bind(&uuid_list)
        .fetch_all(&pool)
        .await?
        .iter()
        .map(|row| row.get::<Uuid, _>("user_id"))
        .all(|owner_id| owner_id == user_id);

    if !user.is_superuser && !all_owner_ids_match_user {
        return Err(Error::Forbidden)?;
    }

    let sql = format!("DELETE FROM {table} WHERE id = ANY ($1)");
    let query = sqlx::query(&sql)
        .bind(&uuid_list)
        .execute(&pool)
        .await?
        .rows_affected();

    if query == 0 {
        return Err(ServerFnError::new("Nothing deleted"));
    }

    Ok(query)
}

#[component]
pub fn BulkDeleteForm(
    table: &'static str,
    action: Action<BulkDelete, Result<u64, ServerFnError>>,
    checked_items: RwSignal<HashSet<String>>,
) -> impl IntoView {
    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        if let Ok(mut data) = BulkDelete::from_event(&ev) {
            data.items = Some(checked_items.get());
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
        <div>
            <ActionForm action on:submit=handle_submit>
                <input type="hidden" name="table" value=table/>
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
        </div>
    }
}
