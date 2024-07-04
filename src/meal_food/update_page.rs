use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use rust_decimal::Decimal;
use uuid::Uuid;

use super::detail_page::get_meal_food_detail;
use super::model::MealFood;
use crate::component::button::SubmitButton;
use crate::component::input::ValidatedInput;
use crate::component::template::{ErrorComponent, LoadingComponent};
use crate::error_extract::{extract_error_message, process_non_field_errors};
use crate::food::router::MealFoodParam;

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user, error::Error, food::model::Food, meal::model::MealBase,
    setup::get_pool,
};

#[server(endpoint = "meal-food-update")]
async fn meal_food_update(
    meal_food_id: Uuid,
    meal_id: Uuid,
    food_id: Uuid,
    quantity: Decimal,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let food = Food::get_by_id(&pool, food_id)
        .await?
        .ok_or(Error::NotFound)?;
    let quantity = food.data_measurement.to_quantity_modifier(&quantity);

    let meal = MealBase::get_by_id(&pool, meal_id)
        .await?
        .ok_or(Error::NotFound)?;
    meal.can_update(&user).await?;

    MealFood::validate(quantity)?;

    MealFood::update_and_return_meal_id(&pool, meal_food_id, quantity, user.id).await?;

    leptos_axum::redirect(&format!("/food/meals/{}", meal.id));
    Ok(())
}

#[component]
pub fn MealFoodUpdatePage() -> impl IntoView {
    let action = Action::<MealFoodUpdate, _>::server();

    let error = move || extract_error_message(&action);
    let non_field_errors = move || process_non_field_errors(error);

    let params = use_params::<MealFoodParam>();
    let id = move || params.with(|p| p.as_ref().map(|p| p.meal_food_id).unwrap_or_default());

    let resource = Resource::new(id, get_meal_food_detail);
    let response =
        move || resource.and_then(|data| view! { <MealFoodUpdateForm data=data.clone() action/> });

    view! {
        <Title text="Edit Meal Food"/>
        <main class="p-4">
            <div class="grid grid-cols-4 gap-4 md:grid-cols-12">
                <div class="col-span-4">
                    <div class="p-4 bg-white border">
                        <h1 class="mb-2 text-base font-bold">"Edit Meal Food"</h1>
                        {non_field_errors}
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
pub fn MealFoodUpdateForm(
    data: MealFood,
    action: Action<MealFoodUpdate, Result<(), ServerFnError>>,
) -> impl IntoView {
    let error = move || extract_error_message(&action);
    let error = Signal::derive(error);
    view! {
        <header class="mb-4">
            <h1 class="text-xl font-bold">
                <A class="hover:underline" href=format!("/food/{}", data.food_id)>
                    {data.food_name}
                </A>
            </h1>
            <p class="text-base text-gray-500">
                <A class="hover:underline" href=format!("/brands/{}", data.food_id)>
                    {data.brand_name}
                </A>
            </p>
        </header>

        <ActionForm action>
            <input type="hidden" name="meal_food_id" value=data.id.to_string()/>
            <input type="hidden" name="meal_id" value=data.meal_id.to_string()/>
            <input type="hidden" name="food_id" value=data.food_id.to_string()/>

            <ValidatedInput error name="quantity" value=format!("{:.2}", data.data_value)/>
            <SubmitButton loading=action.pending() label="Update Meal Food"/>
        </ActionForm>
    }
}
