use leptos::*;
use leptos_router::*;

use rust_decimal::Decimal;
use uuid::Uuid;

use super::detail_page::get_meal_food_detail;
use super::model::MealFood;
use crate::component::button::SubmitButton;
use crate::component::input::NumberInput;
use crate::component::template::{DetailPageTemplate, ErrorComponent, LoadingComponent};
use crate::food::router::MealFoodParam;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user, error::Error, food::model::FoodQuery, meal::model::Meal,
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

    let food = FoodQuery::get_by_id(&pool, food_id)
        .await?
        .ok_or(Error::NotFound)?;
    let quantity = food.data_measurement.to_quantity_modifier(&quantity);

    let meal = Meal::get_by_id(&pool, meal_id)
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
    let params = use_params::<MealFoodParam>();
    let id = move || params.with(|p| p.as_ref().map(|p| p.meal_food_id).unwrap_or_default());

    let action = Action::<MealFoodUpdate, _>::server();

    let resource = Resource::new(id, get_meal_food_detail);
    let response =
        move || resource.and_then(|data| view! { <MealFoodUpdateForm data=data.clone() action/> });

    view! {
        <DetailPageTemplate title="Edit Meal Food">
            <Transition fallback=LoadingComponent>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors/> }
                }>{response}</ErrorBoundary>
            </Transition>
        </DetailPageTemplate>
    }
}

#[component]
pub fn MealFoodUpdateForm(
    data: MealFood,
    action: Action<MealFoodUpdate, Result<(), ServerFnError>>,
) -> impl IntoView {
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || extract_other_errors(action_value, &["name"]);
    let non_field_errors = move || get_non_field_errors(action_value);
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
        <div class="mb-4 text-red-500 font-bold">{action_error}</div>
        <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>
        <ActionForm action>
            <input type="hidden" name="meal_food_id" value=data.id.to_string()/>
            <input type="hidden" name="meal_id" value=data.meal_id.to_string()/>
            <input type="hidden" name="food_id" value=data.food_id.to_string()/>
            <NumberInput action_value name="quantity" value=format!("{:.2}", data.data_value)/>
            <SubmitButton loading=action_loading label="Update Meal Food"/>

        </ActionForm>
    }
}
