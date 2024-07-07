use leptos::*;
use leptos_router::*;

use uuid::Uuid;

use super::detail_page::get_diet_detail;
use crate::component::button::SubmitButton;
use crate::component::template::{DetailPageTemplate, ErrorComponent, LoadingComponent};
use crate::util::param::UuidParam;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, diet::model::Diet, error::Error, setup::get_pool};

#[server(endpoint = "diet-delete")]
pub async fn diet_delete(diet_id: Uuid, redirect_to: Option<String>) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let diet = Diet::get_by_id(&pool, diet_id)
        .await?
        .ok_or(Error::NotFound)?;
    diet.can_delete(&user)?;

    Diet::delete(&pool, diet.id).await?;

    if let Some(redirect_url) = redirect_to {
        leptos_axum::redirect(&redirect_url);
    }
    Ok(())
}

#[component]
pub fn DietDeletePage() -> impl IntoView {
    let params = use_params::<UuidParam>();
    let id = move || params.with(|p| p.as_ref().map(|p| p.id).unwrap_or_default());

    let action = Action::<DietDelete, _>::server();
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || extract_other_errors(action_value, &["non_field_errors"]);
    let non_field_errors = move || get_non_field_errors(action_value);

    let resource = Resource::new(id, get_diet_detail);

    let response = move || {
        resource.and_then(|data| {
            let id = data.id.to_string();
            let redirect_url = data.diet_day_url();
            view! {
                <p class="mb-4">"Are you sure you wish to delete this diet target?"</p>
                <p class="mb-4">"Ths action cannot be undone."</p>
                <ActionForm action>
                    <input type="hidden" name="diet_id" value=id/>
                    <input type="hidden" name="redirect_to" value=redirect_url/>
                    <SubmitButton loading=action_loading label="Delete"/>
                </ActionForm>
            }
        })
    };

    view! {
        <DetailPageTemplate title="Delete Diet">
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
