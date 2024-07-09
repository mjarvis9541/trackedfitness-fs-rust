use leptos::*;
use leptos_router::*;

use crate::component::button::SubmitButton;
use crate::component::template::{DetailPageTemplate, ErrorComponent, LoadingComponent};
use crate::util::param::get_slug;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

use super::detail_page::get_food_detail;

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, error::Error, food::model::Food, setup::get_pool};

#[server(endpoint = "food-delete")]
pub async fn food_delete(slug: String) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let food = Food::get_by_slug(&pool, &slug)
        .await?
        .ok_or(Error::NotFound)?;
    food.can_delete(&user).await?;

    Food::delete(&pool, food.id).await?;

    leptos_axum::redirect("/food");
    Ok(())
}

#[component]
pub fn FoodDeletePage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || get_slug(&params);

    let action = Action::<FoodDelete, _>::server();
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || {
        extract_other_errors(
            action_value,
            &[
                "name",
                "serving",
                "brand_id",
                "energy",
                "fat",
                "saturates",
                "carbohydrate",
                "sugars",
                "fibre",
                "protein",
                "salt",
            ],
        )
    };
    let non_field_errors = move || get_non_field_errors(action_value);

    let resource = Resource::new(slug, get_food_detail);
    let response = move || {
        resource.and_then(|data| {
            let slug = data.slug.clone();
            view! {
                <p class="mb-4">"Are you sure you wish to delete this food?"</p>
                <p class="mb-4">"Ths action cannot be undone."</p>
                <ActionForm action>
                    <input type="hidden" name="slug" value=slug/>
                    <SubmitButton loading=action_loading label="Delete Food"/>
                </ActionForm>
            }
        })
    };

    view! {
        <DetailPageTemplate title="Delete Food">
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
