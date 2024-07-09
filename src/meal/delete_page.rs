use leptos::*;
use leptos_router::*;

use uuid::Uuid;

use super::detail_page::get_meal_detail;
use crate::component::button::SubmitButton;
use crate::component::template::{DetailPageTemplate, ErrorComponent, LoadingComponent};
use crate::util::param::UuidParam;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, error::Error, meal::model::Meal, setup::get_pool};

#[server(endpoint = "meal-delete")]
pub async fn meal_delete(id: Uuid) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let object = Meal::get_by_id(&pool, id).await?.ok_or(Error::NotFound)?;
    object.can_delete(&user).await?;

    Meal::delete(&pool, id).await?;

    leptos_axum::redirect("/food/meals");
    Ok(())
}

#[component]
pub fn MealDeletePage() -> impl IntoView {
    let params = use_params::<UuidParam>();
    let id = move || params.with(|p| p.as_ref().map(|p| p.id).unwrap_or_default());

    let action = Action::<MealDelete, _>::server();
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || extract_other_errors(action_value, &["name"]);
    let non_field_errors = move || get_non_field_errors(action_value);

    let resource = Resource::new(id, get_meal_detail);

    let response = move || {
        resource.and_then(|data| {
            let id = data.id.to_string();
            view! {
                <p class="mb-4">"Are you sure you wish to delete this meal?"</p>
                <p class="mb-4">"Ths action cannot be undone."</p>
                <ActionForm action>
                    <input type="hidden" name="id" value=id/>
                    <SubmitButton loading=action_loading label="Delete Meal"/>
                </ActionForm>
            }
        })
    };

    view! {
        <DetailPageTemplate title="Delete Meal">
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
