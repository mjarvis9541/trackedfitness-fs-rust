use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use uuid::Uuid;

use super::detail_page::get_meal_food_detail;
use super::model::MealFood;
use crate::component::button::SubmitButton;
use crate::component::template::{ErrorComponent, LoadingComponent};
use crate::error_extract::{extract_error_message, process_non_field_errors};
use crate::food::router::MealFoodParam;

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user, error::Error, meal::model::MealBase, setup::get_pool,
};

#[server(endpoint = "meal-food-delete")]
pub async fn meal_food_delete(meal_id: Uuid, meal_food_id: Uuid) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    let meal = MealBase::get_by_id(&pool, meal_id)
        .await?
        .ok_or(Error::NotFound)?;
    meal.can_update(&user).await?;
    let meal_food = MealFood::get_object_or_404(&pool, meal_food_id).await?;
    MealFood::delete(&pool, meal_food.id).await?;
    leptos_axum::redirect("/food/meals");
    Ok(())
}

#[component]
pub fn MealFoodDeletePage() -> impl IntoView {
    let action = Action::<MealFoodDelete, _>::server();

    let error = move || extract_error_message(&action);
    let non_field_errors = move || process_non_field_errors(error);

    let params = use_params::<MealFoodParam>();
    let id = move || params.with(|p| p.as_ref().map(|p| p.meal_food_id).unwrap_or_default());

    let resource = Resource::new(id, get_meal_food_detail);
    let response =
        move || resource.and_then(|data| view! { <MealFoodDeleteForm data=data.clone() action/> });

    view! {
        <Title text="Delete Meal Food"/>
        <main class="p-4">
            <div class="grid grid-cols-4 gap-4 md:grid-cols-12">
                <div class="col-span-4">
                    <div class="p-4 bg-white border">
                        <h1 class="mb-2 text-base font-bold">"Delete Meal Food"</h1>
                        {error}
                        {non_field_errors}
                        <p class="mb-4">"Are you sure you wish to delete this meal food?"</p>
                        <p class="mb-4">"Ths action cannot be undone."</p>
                        <Transition fallback=LoadingComponent>
                            <ErrorBoundary fallback=|errors| {
                                view! { <ErrorComponent errors/> }
                            }>{response}</ErrorBoundary>
                        </Transition>
                    </div>
                </div>
            </div>
        </main>
    }
}

#[component]
pub fn MealFoodDeleteForm(
    data: MealFood,
    action: Action<MealFoodDelete, Result<(), ServerFnError>>,
) -> impl IntoView {
    view! {
        <header class="mb-4">
            <h1 class="text-xl font-bold">
                <A class="hover:underline" href=format!("/food/{}", data.food_slug)>
                    {data.food_name}
                </A>
            </h1>
            <p class="text-base text-gray-500">
                <A class="hover:underline" href=format!("/brands/{}", data.food_slug)>
                    {data.brand_name}
                </A>
            </p>
        </header>

        <ActionForm action>
            <input type="hidden" name="meal_id" value=data.meal_id.to_string()/>
            <input type="hidden" name="meal_food_id" value=data.id.to_string()/>
            <SubmitButton loading=action.pending() label="Delete Meal Food"/>
        </ActionForm>
    }
}
