use leptos::*;
use leptos_router::*;

use chrono::prelude::*;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::component::button::SubmitButton;
use crate::component::input::ValidatedInput;
use crate::error_extract::{extract_error_message, process_non_field_errors};
use crate::meal_of_day::select::MealOfDaySelect;

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
    let action = Action::<FoodToDietCreate, _>::server();
    let loading = action.pending();

    let date: String = Utc::now().date_naive().to_string();

    let error = move || extract_error_message(&action);
    let non_field_errors = move || process_non_field_errors(error);
    let error = Signal::derive(error);

    view! {
        <div>{error}</div>
        <div>{non_field_errors}</div>

        <ActionForm action>
            <input type="hidden" name="food_id" value=food_id/>
            <ValidatedInput error name="date" input_type="date" value=date/>
            <MealOfDaySelect/>
            <ValidatedInput
                error
                name="quantity"
                label=quantity
                input_type="number"
                value=data_value.to_string()
                step="0.01"
            />

            <div class="mt-2">
                <SubmitButton loading label="Add to Diet Log"/>
            </div>
        </ActionForm>
    }
}
