use leptos::*;
use leptos_router::*;

use uuid::Uuid;

use super::detail_page::get_meal_detail;
use crate::component::button::SubmitButton;
use crate::component::input::TextInput;
use crate::component::template::{DetailPageTemplate, ErrorComponent, LoadingComponent};
use crate::util::param::UuidParam;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, error::Error, meal::model::Meal, setup::get_pool};

#[server(endpoint = "meal-update")]
pub async fn meal_update(id: Uuid, name: String) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let object = Meal::get_by_id(&pool, id).await?.ok_or(Error::NotFound)?;
    object.can_update(&user).await?;

    Meal::validate(&name)?;

    let meal = Meal::update(&pool, object.id, &name, user.id).await?;

    leptos_axum::redirect(&format!("/food/meals/{}", meal.id));
    Ok(())
}

#[component]
pub fn MealUpdatePage() -> impl IntoView {
    let params = use_params::<UuidParam>();
    let id = move || params.with(|p| p.as_ref().map(|p| p.id).unwrap_or_default());

    let action = Action::<MealUpdate, _>::server();
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || extract_other_errors(action_value, &["name"]);
    let non_field_errors = move || get_non_field_errors(action_value);

    let resource = Resource::new(id, get_meal_detail);
    let response = move || {
        resource.and_then(|data| {
            let id = data.id.to_string();
            let name = data.name.clone();
            view! {
                <ActionForm action>
                    <input type="hidden" name="id" value=id/>
                    <TextInput action_value name="name" placeholder="Enter meal name" value=name/>
                    <SubmitButton loading=action_loading label="Update Meal"/>
                </ActionForm>
            }
        })
    };
    view! {
        <DetailPageTemplate title="Edit Meal">
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
