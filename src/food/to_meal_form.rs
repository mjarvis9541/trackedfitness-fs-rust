use leptos::*;
use leptos_router::*;

use rust_decimal::Decimal;
use uuid::Uuid;

use crate::component::button::SubmitButton;
use crate::component::input::NumberInput;
use crate::meal::select::MealSelect;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user, error::Error, meal::model::Meal, meal_food::model::MealFood,
    setup::get_pool,
};

#[server(endpoint = "food-to-meal-food-create")]
pub async fn food_to_meal_create(
    meal_id: Uuid,
    food_id: Uuid,
    quantity: Decimal,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    let meal = Meal::get_by_id(&pool, meal_id)
        .await?
        .ok_or(Error::NotFound)?;
    meal.can_update(&user).await?;
    MealFood::validate(quantity)?;
    MealFood::create_and_return_meal_id(&pool, meal.id, food_id, quantity, user.id).await?;
    leptos_axum::redirect(&format!("/food/meals/{}", meal.id));
    Ok(())
}

#[component]
pub fn FoodToMealForm(food_id: String, data_value: i32, quantity: String) -> impl IntoView {
    let _quantity = quantity;
    let action = Action::<FoodToMealCreate, _>::server();

    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || extract_other_errors(action_value, &["name"]);
    let non_field_errors = move || get_non_field_errors(action_value);

    view! {
        <div class="mb-4 text-red-500 font-bold">{action_error}</div>
        <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>
        <ActionForm action>
            <input type="hidden" name="food_id" value=food_id/>
            <MealSelect/>
            <NumberInput action_value name="quantity" value=data_value.to_string() step="0.01"/>
            <SubmitButton loading=action_loading label="Add to Meal"/>
        </ActionForm>
    }
}
