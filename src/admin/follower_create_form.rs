use leptos::*;
use leptos_router::*;

use uuid::Uuid;

use super::user_select::UserSelect;
use crate::component::button::SubmitButton;
use crate::component::select::{FieldSelect, FOLLOWER_STATUS_FORM_OPTIONS};

#[server(endpoint = "admin-follower-create")]
pub async fn admin_follower_create(
    user_id: Uuid,
    follower_id: Uuid,
    status: i32,
) -> Result<(), ServerFnError> {
    crate::auth::service::extract_superuser_from_request()?;
    let pool = crate::setup::get_pool()?;
    crate::follower::model::Follower::create(&pool, user_id, follower_id, status).await?;
    Ok(())
}

#[component]
pub fn AdminFollowerCreateForm(
    action: Action<AdminFollowerCreate, Result<(), ServerFnError>>,
) -> impl IntoView {
    view! {
        <ActionForm action>
            <UserSelect name="user_id" label="user"/>
            <UserSelect name="follower_id" label="follower"/>
            <FieldSelect name="status" options=&FOLLOWER_STATUS_FORM_OPTIONS/>
            <SubmitButton loading=action.pending() label="Create Follower"/>
        </ActionForm>
    }
}
