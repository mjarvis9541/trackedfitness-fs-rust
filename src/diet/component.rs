use leptos::*;

use super::model::{DietFoodQuery, FormattedFoodData};

// const TD_LEFT: &'static str = "p-2 text-left border";
// const TD_RIGHT: &'static str = "p-2 text-right border";

#[component]
pub fn NutritionInformation(data: DietFoodQuery) -> impl IntoView {
    let formatted = data.format();

    view! {
        <h3 class="mb-2 font-bold">"Nutrition Information"</h3>
        <table class="mb-4 w-full border-collapse">
            <thead>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Typical Values"</th>
                    <th class="p-2 w-1/2 text-right border">"Per serving"</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <th class="p-2 text-left border">"Energy (kcal)"</th>
                    <td class="p-2 text-right border">{formatted.energy}</td>
                </tr>
                <tr>
                    <th class="p-2 text-left border">"Protein"</th>
                    <td class="p-2 text-right border">{formatted.protein}</td>
                </tr>
                <tr>
                    <th class="p-2 text-left border">"Carbohydrate"</th>
                    <td class="p-2 text-right border">{formatted.carbohydrate}</td>
                </tr>
                <tr>
                    <th class="p-2 text-left border">"Fat"</th>
                    <td class="p-2 text-right border">{formatted.fat}</td>
                </tr>
                <tr>
                    <th class="p-2 text-left border">"Sat. Fat"</th>
                    <td class="p-2 text-right border">{formatted.saturates}</td>
                </tr>
                <tr>
                    <th class="p-2 text-left border">"Sugars"</th>
                    <td class="p-2 text-right border">{formatted.sugars}</td>
                </tr>
                <tr>
                    <th class="p-2 text-left border">"Fibre"</th>
                    <td class="p-2 text-right border">{formatted.fibre}</td>
                </tr>
                <tr>
                    <th class="p-2 text-left border">"Salt"</th>
                    <td class="p-2 text-right border">{formatted.salt}</td>
                </tr>
            </tbody>
        </table>
    }
}

const HEADER_CSS: &str = "hidden justify-end items-end p-2 text-xs text-gray-500 border-b lg:flex";

#[component]
pub fn DietMealGridHeader() -> impl IntoView {
    let headers = vec![
        "Quantity", "Calories", "Protein", "Carbs", "Fat", "Sat. Fat", "Sugars", "Fibre", "Salt",
    ];
    headers
        .into_iter()
        .map(|header| view! { <div class=HEADER_CSS>{header}</div> })
        .collect_view()
}

const GRID_HEADER_CSS: &str =
    "flex px-2 text-xs md:hidden group-hover:bg-amber-200 group-odd:bg-gray-50";

#[component]
pub fn DietFoodGridHeader() -> impl IntoView {
    view! {
        <div class=GRID_HEADER_CSS>"Calories"</div>
        <div class=GRID_HEADER_CSS>"Protein"</div>
        <div class=GRID_HEADER_CSS>"Carbs"</div>
        <div class=GRID_HEADER_CSS>"Fat"</div>
    }
}

const ROW_CSS: &str =
    "flex items-center justify-end p-2 border-b group-hover:bg-amber-200 group-odd:bg-gray-50";

#[component]
pub fn DietTotalRow(formatted: FormattedFoodData) -> impl IntoView {
    view! {
        <div class=ROW_CSS>{formatted.energy}</div>
        <div class=ROW_CSS>{formatted.protein}</div>
        <div class=ROW_CSS>{formatted.carbohydrate}</div>
        <div class=ROW_CSS>{formatted.fat}</div>
        <div class=ROW_CSS>{formatted.saturates}</div>
        <div class=ROW_CSS>{formatted.sugars}</div>
        <div class=ROW_CSS>{formatted.fibre}</div>
        <div class=ROW_CSS>{formatted.salt}</div>
    }
}
