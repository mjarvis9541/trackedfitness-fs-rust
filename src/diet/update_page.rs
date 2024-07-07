use leptos::*;
use leptos_router::*;

use chrono::prelude::*;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::component::button::SubmitButton;
use crate::component::input::{NumberInput, TextInput};
use crate::component::template::{DetailPageTemplate, ErrorComponent, LoadingComponent};
use crate::diet::detail_page::get_diet_detail;
use crate::meal_of_day::select::MealOfDaySelect;
use crate::util::param::UuidParam;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user, diet::model::Diet, error::Error, food::model::Food,
    setup::get_pool,
};

#[server(endpoint = "diet-update")]
pub async fn diet_update(
    id: Uuid,
    username: String,
    date: NaiveDate,
    meal_of_day_id: Uuid,
    food_id: Uuid,
    quantity: Decimal,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let diet = Diet::get_by_id(&pool, id).await?.ok_or(Error::NotFound)?;
    diet.can_update(&user)?;
    Diet::validate(date, quantity)?;

    let food = Food::get_by_id(&pool, food_id)
        .await?
        .ok_or(Error::NotFound)?;

    let quantity = food.data_measurement.to_quantity_modifier(&quantity);

    Diet::update(&pool, diet.id, date, meal_of_day_id, quantity, user.id).await?;
    leptos_axum::redirect(&format!("/users/{username}/diet/{date}"));
    Ok(())
}

#[component]
pub fn DietUpdatePage() -> impl IntoView {
    let params = use_params::<UuidParam>();
    let id = move || params.with(|p| p.as_ref().map_or_else(|_| Uuid::default(), |p| p.id));

    let action = Action::<DietUpdate, _>::server();
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || {
        extract_other_errors(
            action_value,
            &[
                "non_field_errors",
                "username",
                "food_id",
                "id",
                "date",
                "quantity",
            ],
        )
    };
    let non_field_errors = move || get_non_field_errors(action_value);

    let resource = Resource::new(id, get_diet_detail);
    let response = move || {
        resource.and_then(|data| {
            let data = data.clone();
            let title = data.title();
            view! {
                <h1 class="mb-2 text-xl font-bold">{title}</h1>
                <p class="mb-4 font-bold capitalize">{data.brand_name}</p>

                <ActionForm action>
                    <input type="hidden" name="username" value=data.username/>
                    <input type="hidden" name="food_id" value=data.food_id.to_string()/>
                    <input type="hidden" name="id" value=data.id.to_string()/>
                    <TextInput
                        action_value
                        name="date"
                        input_type="date"
                        value=data.date.to_string()
                    />
                    <MealOfDaySelect selected=data.meal_of_day_id/>
                    <NumberInput
                        action_value
                        name="quantity"
                        value=format!("{:.2}", data.data_value)
                        step="0.01"
                    />
                    <SubmitButton loading=action_loading label="Update Diet Log"/>
                </ActionForm>
            }
        })
    };

    view! {
        <DetailPageTemplate title="Edit Diet Food">
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
