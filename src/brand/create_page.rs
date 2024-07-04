use leptos::*;

use leptos_router::*;

use crate::component::button::SubmitButton;
use crate::component::input::TextInputImproved;
use crate::component::template::DetailPageTemplate;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, brand::model::Brand, setup::get_pool};

#[server(endpoint = "brand-create")]
pub async fn brand_create(name: String) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    Brand::can_create(&user).await?;
    Brand::validate(&name)?;

    let object = Brand::create(&pool, &name, user.id).await?;

    leptos_axum::redirect(&format!("/food/brands/{}", object.slug));
    Ok(())
}

#[component]
pub fn BrandCreatePage() -> impl IntoView {
    let action = Action::<BrandCreate, _>::server();
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || extract_other_errors(action_value, &["name"]);
    let non_field_errors = move || get_non_field_errors(action_value);

    view! {
        <DetailPageTemplate title="New Brand">
            <div class="mb-4 text-red-500 font-bold">{action_error}</div>
            <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>
            <ActionForm action>
                <TextInputImproved action_value name="name" placeholder="Enter brand name"/>
                <SubmitButton loading=action_loading label="Create Brand"/>
            </ActionForm>
        </DetailPageTemplate>
    }
}
