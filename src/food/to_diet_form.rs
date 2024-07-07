use leptos::*;
use leptos_router::*;

use chrono::prelude::*;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::component::button::SubmitButton;
use crate::component::input::{NumberInput, TextInput};
use crate::meal_of_day::select::MealOfDaySelect;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user, diet::model::Diet, error::Error, food::model::Food,
    setup::get_pool,
};

#[server(endpoint = "food-to-diet-create")]
pub async fn food_to_diet_create(
    date: NaiveDate,
    meal_of_day_id: Uuid,
    food_id: Uuid,
    quantity: Decimal,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    Diet::can_create(&user, user.id)?;

    Diet::validate(date, quantity)?;

    let food = Food::get_by_id(&pool, food_id)
        .await?
        .ok_or(Error::NotFound)?;
    let quantity = food.data_measurement.to_quantity_modifier(&quantity);

    Diet::create(
        &pool,
        date,
        user.id,
        meal_of_day_id,
        food.id,
        quantity,
        user.id,
    )
    .await?;

    leptos_axum::redirect(&format!("/users/{}/diet/{}", user.username, date));
    Ok(())
}

#[component]
pub fn FoodToDietForm(food_id: String, data_value: i32, quantity: String) -> impl IntoView {
    let _quantity = quantity;
    let action = Action::<FoodToDietCreate, _>::server();

    let date: String = Utc::now().date_naive().to_string();

    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || extract_other_errors(action_value, &["name"]);
    let non_field_errors = move || get_non_field_errors(action_value);

    view! {
        <div class="mb-4 text-red-500 font-bold">{action_error}</div>
        <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>

        <ActionForm action>
            <input type="hidden" name="food_id" value=food_id/>
            <MealOfDaySelect/>

            <TextInput action_value name="date" input_type="date" value=date/>

            <NumberInput action_value name="quantity" value=data_value.to_string() step="0.01"/>

            <SubmitButton loading=action_loading label="Add to Diet Log"/>

        </ActionForm>
    }
}
