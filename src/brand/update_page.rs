use leptos::*;
use leptos_router::*;

use uuid::Uuid;

use crate::component::button::SubmitButton;
use crate::component::input::TextInput;
use crate::component::template::{DetailPageTemplate, ErrorComponent, LoadingComponent};
use crate::util::param::get_slug;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

use super::detail_page::get_brand_detail;

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, brand::model::Brand, error::Error, setup::get_pool};

#[server(endpoint = "brand-update")]
pub async fn brand_update(id: Uuid, name: String) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    let object = Brand::get_by_id(&pool, id).await?.ok_or(Error::NotFound)?;
    object.can_view(&user)?;
    Brand::validate(&name)?;
    let updated = Brand::update(&pool, object.id, &name, user.id).await?;
    leptos_axum::redirect(&format!("/food/brands/{}", updated.slug));
    Ok(())
}

#[component]
pub fn BrandUpdatePage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || get_slug(&params);

    let action = Action::<BrandUpdate, _>::server();
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || extract_other_errors(action_value, &["name"]);
    let non_field_errors = move || get_non_field_errors(action_value);

    let resource = Resource::new(slug, get_brand_detail);

    let response = move || {
        resource.and_then(|data| {
            let id = data.id.to_string();
            let name = data.name.clone();
            view! {
                <ActionForm action>
                    <input type="hidden" name="id" value=id/>
                    <TextInput action_value name="name" value=name placeholder="Enter brand name"/>
                    <SubmitButton loading=action_loading label="Edit Brand"/>
                </ActionForm>
            }
        })
    };

    view! {
        <DetailPageTemplate title="Edit Brand">
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
