use leptos::{component, view, IntoView};

use crate::food::model::Nutrition;

#[component]
pub fn NutritionRow<'a>(data: &'a Nutrition) -> impl IntoView {
    view! {
        <div class="flex justify-end items-center py-2 px-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
            {format!("{:.0}", data.energy)} "kcal"
        </div>
        <div class="flex justify-end items-center py-2 px-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
            {format!("{:.1}", data.protein)} "g"
        </div>
        <div class="flex justify-end items-center py-2 px-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
            {format!("{:.1}", data.carbohydrate)} "g"
        </div>
        <div class="flex justify-end items-center py-2 px-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
            {format!("{:.1}", data.fat)} "g"
        </div>
        <div class="hidden justify-end items-center py-2 px-2 lg:flex group-hover:bg-gray-200 group-odd:bg-gray-50">
            {format!("{:.1}", data.saturates)} "g"
        </div>
        <div class="hidden justify-end items-center py-2 px-2 lg:flex group-hover:bg-gray-200 group-odd:bg-gray-50">
            {format!("{:.1}", data.sugars)} "g"
        </div>
        <div class="hidden justify-end items-center py-2 px-2 lg:flex group-hover:bg-gray-200 group-odd:bg-gray-50">
            {format!("{:.1}", data.fibre)} "g"
        </div>
        <div class="hidden justify-end items-center py-2 px-2 lg:flex group-hover:bg-gray-200 group-odd:bg-gray-50">
            {format!("{:.2}", data.salt)} "g"
        </div>
    }
}
