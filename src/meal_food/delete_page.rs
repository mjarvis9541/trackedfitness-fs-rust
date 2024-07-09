use leptos::*;
use leptos_router::*;

use uuid::Uuid;

use super::detail_page::get_meal_food_detail;
use crate::component::button::SubmitButton;
use crate::component::template::{DetailPageTemplate, ErrorComponent, LoadingComponent};
use crate::food::router::MealFoodParam;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user, error::Error, meal::model::Meal, meal_food::model::MealFood,
    setup::get_pool,
};

#[server(endpoint = "meal-food-delete")]
pub async fn meal_food_delete(meal_id: Uuid, meal_food_id: Uuid) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    let meal = Meal::get_by_id(&pool, meal_id)
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
    let params = use_params::<MealFoodParam>();
    let id = move || params.with(|p| p.as_ref().map(|p| p.meal_food_id).unwrap_or_default());

    let action = Action::<MealFoodDelete, _>::server();
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || extract_other_errors(action_value, &["name"]);
    let non_field_errors = move || get_non_field_errors(action_value);

    let resource = Resource::new(id, get_meal_food_detail);
    let response = move || {
        resource.and_then(|data| {
            let food_name = data.food_name.clone();
            let brand_name = data.brand_name.clone();
            let meal_id = data.meal_id.to_string();
            let meal_food_id = data.id.to_string();
            view! {
                <header class="mb-4">
                    <h1 class="text-xl font-bold">
                        <A class="hover:underline" href=format!("/food/{}", data.food_slug)>
                            {food_name}
                        </A>
                    </h1>
                    <p class="text-gray-600">
                        <A class="hover:underline" href=format!("/brands/{}", data.food_slug)>
                            {brand_name}
                        </A>
                    </p>
                </header>
                <p class="mb-4">"Are you sure you wish to delete this meal food?"</p>
                <p class="mb-4">"Ths action cannot be undone."</p>
                <div class="mb-4 text-red-500 font-bold">{action_error}</div>
                <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>
                <ActionForm action>
                    <input type="hidden" name="meal_id" value=meal_id/>
                    <input type="hidden" name="meal_food_id" value=meal_food_id/>
                    <SubmitButton loading=action_loading label="Delete Meal Food"/>
                </ActionForm>
            }
        })
    };

    view! {
        <DetailPageTemplate title="Delete Meal Food">

            <Transition fallback=LoadingComponent>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors/> }
                }>{response}</ErrorBoundary>
            </Transition>
        </DetailPageTemplate>
    }
}
