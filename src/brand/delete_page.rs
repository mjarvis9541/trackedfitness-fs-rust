use leptos::*;
use leptos_router::*;

use uuid::Uuid;

use crate::component::button::SubmitButton;
use crate::component::template::{DetailPageTemplate, ErrorComponent, LoadingComponent};
use crate::util::param::get_slug;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

use super::detail_page::get_brand_detail;

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, brand::model::Brand, error::Error, setup::get_pool};

#[server(endpoint = "brand-delete")]
pub async fn brand_delete(id: Uuid) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let object = Brand::get_by_id(&pool, id).await?.ok_or(Error::NotFound)?;
    object.can_delete(&user).await?;

    Brand::delete(&pool, object.id).await?;

    leptos_axum::redirect("/food/brands");
    Ok(())
}

#[component]
pub fn BrandDeletePage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || get_slug(&params);

    let action = Action::<BrandDelete, _>::server();
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || extract_other_errors(action_value, &["name"]);
    let non_field_errors = move || get_non_field_errors(action_value);

    let resource = Resource::new(slug, get_brand_detail);

    let response = move || {
        resource.and_then(|data| {
            let id = data.id.to_string();
            view! {
                <p class="mb-4">"Are you sure you wish to delete this brand?"</p>
                <p class="mb-4">"Ths action cannot be undone."</p>
                <ActionForm action>
                    <input type="hidden" name="id" value=id/>
                    <SubmitButton loading=action_loading label="Delete Brand"/>
                </ActionForm>
            }
        })
    };

    view! {
        <DetailPageTemplate title="Delete Brand">
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
