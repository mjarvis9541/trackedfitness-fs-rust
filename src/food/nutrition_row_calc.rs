use leptos::*;
use leptos_router::*;

use rust_decimal::Decimal;

use super::data_measurement::DataMeasurement;
use super::model::Food;

use crate::component::template::FoodListItemMacroHeader;

pub fn calculate_macronutrients(
    quantity: RwSignal<Decimal>,
    nutrient: Decimal,
    data_value_decimal: Decimal,
) -> impl Fn() -> String {
    move || {
        quantity.with(|q| {
            if data_value_decimal.is_zero() {
                "0".to_string()
            } else {
                format!("{:.1}", *q * nutrient / data_value_decimal)
            }
        })
    }
}

#[component]
pub fn QuantityInput(
    #[prop(into)] data_value: RwSignal<Decimal>,
    #[prop(into)] data_measurement: RwSignal<DataMeasurement>,
) -> impl IntoView {
    let is_valid_input = RwSignal::new(true);

    let class = move || {
        match is_valid_input.with(|value| *value) {
            true => "p-2 w-full border focus:border-blue-500 focus:ring-2 focus:ring-blue-500 focus:outline-none",
            false => "p-2 w-full border focus:border-red-500 focus:ring-2 focus:ring-red-500 focus:outline-none"
        }
    };

    let handle_input = move |ev| match Decimal::from_str_exact(&event_target_value(&ev)) {
        Ok(value) if value.is_sign_negative() => {
            is_valid_input.update(|v| *v = false);
        }
        Ok(value) if value > Decimal::from(1000) => {
            is_valid_input.update(|v| *v = false);
        }
        Ok(value) => {
            is_valid_input.update(|v| *v = true);
            data_value.update(|prev| *prev = value)
        }
        Err(_) => {
            is_valid_input.update(|v| *v = false);
            data_value.update(|prev| *prev = Decimal::from(0))
        }
    };

    let form_step = move || data_measurement.with(|v| v.to_form_step());
    let form_value = move || data_measurement.with(|v| v.to_form_value(data_value));
    let form_label = move || data_measurement.with(|v| v.to_string());

    view! {
        <div class="flex relative justify-end items-center">
            <input
                type="number"
                name="quantity"
                step=form_step
                value=form_value
                on:input=handle_input
                min=0
                max=1000
                class=class
            />
            <div class="absolute pr-10 text-gray-400 pointer-events-none">{form_label}</div>
        </div>
    }
}

#[component]
pub fn FoodNutritionCalculationRow(
    data: Food,
    data_value_decimal: Decimal,
    quantity: RwSignal<Decimal>,
) -> impl IntoView {
    let energy = calculate_macronutrients(quantity, Decimal::from(data.energy), data_value_decimal);
    let protein = calculate_macronutrients(quantity, data.protein, data_value_decimal);
    let carbohydrate = calculate_macronutrients(quantity, data.carbohydrate, data_value_decimal);
    let fat = calculate_macronutrients(quantity, data.fat, data_value_decimal);
    let saturates = calculate_macronutrients(quantity, data.saturates, data_value_decimal);
    let sugars = calculate_macronutrients(quantity, data.sugars, data_value_decimal);
    let fibre = calculate_macronutrients(quantity, data.fibre, data_value_decimal);
    let salt = calculate_macronutrients(quantity, data.salt, data_value_decimal);

    let data_measurement = RwSignal::new(data.data_measurement);

    view! {
        <div class="col-span-3 py-1 px-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
            <div class="overflow-hidden font-bold md:font-normal truncate">
                <A href=format!("/food/{}", data.slug)>{data.name}</A>
            </div>
            <div class="text-xs">
                <A href=format!("/brands/{}", data.brand_slug)>{data.brand_name}</A>
            </div>
        </div>

        <div class="flex justify-center items-center group-hover:bg-gray-200 group-odd:bg-gray-50">
            <QuantityInput data_measurement=data_measurement data_value=quantity/>
        </div>

        <FoodListItemMacroHeader/>

        <div class="flex justify-end items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
            {energy} "kcal"
        </div>
        <div class="flex justify-end items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
            {protein}
        </div>
        <div class="flex justify-end items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
            {carbohydrate}
        </div>
        <div class="flex justify-end items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
            {fat}
        </div>
        <div class="hidden justify-end items-center p-2 lg:flex group-hover:bg-gray-200 group-odd:bg-gray-50">
            {saturates}
        </div>
        <div class="hidden justify-end items-center p-2 lg:flex group-hover:bg-gray-200 group-odd:bg-gray-50">
            {sugars}
        </div>
        <div class="hidden justify-end items-center p-2 lg:flex group-hover:bg-gray-200 group-odd:bg-gray-50">
            {fibre}
        </div>
        <div class="hidden justify-end items-center p-2 lg:flex group-hover:bg-gray-200 group-odd:bg-gray-50">
            {salt}
        </div>
    }
}
