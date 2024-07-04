use leptos::*;
use leptos_router::*;

use uuid::Uuid;

use crate::component::button::SubmitButton;
use crate::component::template::{DetailPageTemplate, ErrorComponent, LoadingComponent};
use crate::util::param::get_slug;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

use super::detail_page::get_movement_detail;

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user, error::Error, movement::model::MovementBase, setup::get_pool,
};

#[server(endpoint = "movement-delete")]
pub async fn movement_delete(id: Uuid) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let object = MovementBase::get_by_id(&pool, id)
        .await?
        .ok_or(Error::NotFound)?;
    object.can_delete(&user).await?;

    MovementBase::delete(&pool, object.id).await?;

    leptos_axum::redirect("/movement");
    Ok(())
}

#[component]
pub fn MovementDeletePage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || get_slug(&params);

    let action = Action::<MovementDelete, _>::server();
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || extract_other_errors(action_value, &["name"]);
    let non_field_errors = move || get_non_field_errors(action_value);

    let resource = Resource::new(slug, get_movement_detail);
    let response = move || {
        resource.and_then(|data| {
            let id = data.id.to_string();
            view! {
                <p class="mb-4">"Are you sure you wish to delete this exercise?"</p>
                <p class="mb-4">"Ths action cannot be undone."</p>
                <ActionForm action>
                    <input type="hidden" name="id" value=id/>
                    <SubmitButton loading=action_loading label="Delete Exercise"/>
                </ActionForm>
            }
        })
    };

    view! {
        <DetailPageTemplate title="Delete Exercise">
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
