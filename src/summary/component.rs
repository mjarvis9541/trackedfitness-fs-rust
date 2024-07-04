use std::collections::HashSet;

use chrono::Utc;
use leptos::*;

use crate::component::checkbox::CheckboxListItem;

use super::model::{UserDaySummary, Variant};

#[component]
pub fn DaySummaryListItem(
    data: UserDaySummary,
    #[prop(optional)] checked_items: RwSignal<HashSet<String>>,
    #[prop(optional)] total_row: bool,
    #[prop(default = Variant::DietLog)] variant: Variant,
) -> impl IntoView {
    let row_css = if data.date == Utc::now().date_naive() {
        "p-2 border-b flex items-center justify-end bg-blue-500/25"
    } else if data.actual {
        "p-2 border-b flex items-center justify-end group-hover:bg-amber-200 group-odd:bg-gray-50"
    } else if total_row {
        "p-2 border-b flex items-center justify-end bg-gray-200 font-bold border-t"
    } else {
        "p-2 border-b flex items-center justify-end group-hover:bg-amber-200 group-odd:bg-gray-50"
    };

    let row_header = if total_row {
        view! {
            <div class="p-2 border-b flex items-center bg-gray-200 font-bold col-span-3">
                "Week Average"
            </div>
        }
        .into_view()
    } else {
        view! {
            <div class=row_css>
                <CheckboxListItem id=data.date.to_string() checked_items/>
            </div>
            <div class=row_css>
                <a class="text-blue-500 hover:underline" href=data.generate_main_link(variant)>
                    {data.date.format("%d/%m/%Y").to_string()}
                </a>
            </div>
            <div class=row_css>{data.date.format("%A").to_string()}</div>
        }
        .into_view()
    };

    view! {
        <div class="contents group">
            {row_header} <div class=row_css>{format!("{:.0}kcal", data.energy)}</div>
            <div class=row_css>
                {format!("{:.1}", data.protein)}
                <span class="ml-1 text-xs text-gray-400">
                    {format!("({:.0}%)", data.protein_pct)}
                </span>
            </div>
            <div class=row_css>
                {format!("{:.1}", data.carbohydrate)}
                <span class="ml-1 text-xs text-gray-400">
                    {format!("({:.0}%)", data.carbohydrate_pct)}
                </span>
            </div>
            <div class=row_css>
                {format!("{:.1}", data.fat)}
                <span class="ml-1 text-xs text-gray-400">{format!("({:.0}%)", data.fat_pct)}</span>
            </div> <div class=row_css>{format!("{:.1}", data.saturates)}</div>
            <div class=row_css>{format!("{:.1}", data.sugars)}</div>
            <div class=row_css>{format!("{:.1}", data.fibre)}</div>
            <div class=row_css>{format!("{:.2}", data.salt)}</div>
            <div class=row_css>{format!("{:.0}kcal", data.energy_per_kg)}</div>
            <div class=row_css>{format!("{:.2}", data.protein_per_kg)}</div>
            <div class=row_css>{format!("{:.2}", data.carbohydrate_per_kg)}</div>
            <div class=row_css>{format!("{:.2}", data.fat_per_kg)}</div>
            <div class=row_css>{format!("{:.2}", data.weight)}</div>
        </div>
    }
}
