use leptos::*;
use leptos_router::*;

use super::user_select::UserSelect;
use crate::component::button::SubmitButton;
use crate::component::select::{FieldSelect, FORM_BLOCKED_STATUS_OPTIONS};

use uuid::Uuid;

#[server]
async fn admin_user_block_create(
    blocker_id: Uuid,
    blocked_id: Uuid,
    blocked_status: String,
) -> Result<(), ServerFnError> {
    crate::auth::service::extract_superuser_from_request()?;
    let pool = expect_context::<sqlx::PgPool>();
    let status = blocked_status.parse::<i32>().unwrap_or_default();
    crate::user_block::model::UserBlock::create(&pool, blocker_id, blocked_id, status).await?;
    Ok(())
}

#[component]
pub fn AdminUserBlockCreateForm(
    action: Action<AdminUserBlockCreate, Result<(), ServerFnError>>,
) -> impl IntoView {
    view! {
        <ActionForm action>
            <UserSelect name="blocker_id" label="blocker"/>
            <UserSelect name="blocked_id" label="blocked"/>
            <FieldSelect name="blocked_status" options=&FORM_BLOCKED_STATUS_OPTIONS/>
            <SubmitButton loading=action.pending() label="Create Blocked User"/>
        </ActionForm>
    }
}
