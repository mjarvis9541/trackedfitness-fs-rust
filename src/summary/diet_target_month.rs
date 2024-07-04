use std::collections::HashSet;

use chrono::NaiveDate;
use leptos::*;
use leptos_router::*;

use crate::component::checkbox::CheckboxListItem;
use crate::component::template::{AutoListHeader, ErrorComponent, ListLoadingComponent};
use crate::util::param::{get_date, get_username};

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user,
    setup::get_pool,
    util::datetime::{get_month_end_comprehensive, get_month_start_comprehensive},
};

use super::model::{UserDaySummary, UserWeekSummary, Variant};

#[server]
pub async fn get_diet_target_month_summary(
    username: String,
    date: NaiveDate,
) -> Result<UserWeekSummary, ServerFnError> {
    let _user = get_request_user()?;
    let pool = get_pool()?;
    let start = get_month_start_comprehensive(date);
    let end = get_month_end_comprehensive(date);
    let query = UserDaySummary::get_target_range(&pool, &username, start, end).await?;
    dbg!(&query);

    let complete_day_totals = UserDaySummary::fill_missing_days_with_previous(query, start, end)?;
    // dbg!("completed totals {:?}", &complete_day_totals);

    let total_days = complete_day_totals.len() as i32;
    let week_avg = UserDaySummary::calculate_averages(&complete_day_totals, total_days)?;

    Ok(UserWeekSummary {
        day_total_vec: complete_day_totals,
        period_avg: week_avg,
    })
}

#[component]
pub fn DietTargetWeekSummaryComponent() -> impl IntoView {
    let params = use_params_map();
    let username = move || get_username(&params);
    let date = move || get_date(&params);
    let week_resource = Resource::new(
        move || (username(), date()),
        |(username, date)| get_diet_target_month_summary(username, date),
    );

    let all_items = RwSignal::new(HashSet::<String>::new());
    let checked_items = RwSignal::new(HashSet::<String>::new());
    let week_response = move || {
        week_resource.and_then(|data| {
            let ids: HashSet<String> = data
                .day_total_vec
                .iter()
                .map(|item| item.date.to_string())
                .collect();
            all_items.update(|set| set.extend(ids));
            let day_view = data
                .day_total_vec
                .iter()
                .map(|day_item| view! { <DaySummaryMonthListItem data=day_item.clone() checked_items/> })
                .collect_view();

            view! {
                {day_view}
                <DaySummaryMonthAverageComponent data=data.period_avg.clone()/>
            }
        })
    };
    view! {
        <div>
            <div class="grid grid-cols-checkbox-16">
                <AutoListHeader all_items checked_items>
                    "Date"
                    "Day"
                    "Calories"
                    "Protein"
                    "Carbs"
                    "Fat"
                    "Sat. Fat"
                    "Sugars"
                    "Fibre"
                    "Salt"
                    "Cals/kg"
                    "Pro/kg"
                    "Carbs/kg"
                    "Fat/kg"
                    "Weight"
                </AutoListHeader>
                <Transition fallback=ListLoadingComponent>
                    <ErrorBoundary fallback=|errors| {
                        view! { <ErrorComponent errors/> }
                    }>{week_response}</ErrorBoundary>
                </Transition>
            </div>
        </div>
    }
}

#[component]
pub fn DaySummaryMonthListItem(
    data: UserDaySummary,
    checked_items: RwSignal<HashSet<String>>,
) -> impl IntoView {
    view! {
        <div
            class="contents group p-2 border-b group-hover:bg-amber-200 group-odd:bg-gray-50"
            class=("font-bold", data.actual)
        >
            <div class="p-2 border-b group-hover:bg-amber-200 group-odd:bg-gray-50">
                <CheckboxListItem id=data.date.to_string() checked_items/>
            </div>
            <div class="p-2 border-b group-hover:bg-amber-200 group-odd:bg-gray-50">
                <a class="text-blue-500" href=data.generate_main_link(Variant::DietLog)>
                    {data.date.format("%d/%m/%Y").to_string()}
                </a>
            </div>
            <div class="p-2 border-b group-hover:bg-amber-200 group-odd:bg-gray-50">
                {data.date.format("%A").to_string()}
            </div>
            <div class="p-2 border-b group-hover:bg-amber-200 group-odd:bg-gray-50 text-end">
                {format!("{:.0}", data.energy)} "kcal"
            </div>
            <div class="p-2 border-b group-hover:bg-amber-200 group-odd:bg-gray-50 text-end">
                {format!("{:.1}", data.protein)}
                <span class="ml-1 text-xs text-gray-400">
                    "(" {format!("{:.0}", data.protein_pct)} "%)"
                </span>
            </div>
            <div class="p-2 border-b group-hover:bg-amber-200 group-odd:bg-gray-50 text-end">
                {format!("{:.1}", data.carbohydrate)}
                <span class="ml-1 text-xs text-gray-400">
                    "(" {format!("{:.0}", data.carbohydrate_pct)} "%)"
                </span>
            </div>
            <div class="p-2 border-b group-hover:bg-amber-200 group-odd:bg-gray-50 text-end">
                {format!("{:.1}", data.fat)}
                <span class="ml-1 text-xs text-gray-400">
                    "(" {format!("{:.0}", data.fat_pct)} "%)"
                </span>
            </div>
            <div class="p-2 border-b group-hover:bg-amber-200 group-odd:bg-gray-50 text-end">
                {format!("{:.1}", data.saturates)}
            </div>
            <div class="p-2 border-b group-hover:bg-amber-200 group-odd:bg-gray-50 text-end">
                {format!("{:.1}", data.sugars)}
            </div>
            <div class="p-2 border-b group-hover:bg-amber-200 group-odd:bg-gray-50 text-end">
                {format!("{:.1}", data.fibre)}
            </div>
            <div class="p-2 border-b group-hover:bg-amber-200 group-odd:bg-gray-50 text-end">
                {format!("{:.2}", data.salt)}
            </div>
            <div class="p-2 border-b group-hover:bg-amber-200 group-odd:bg-gray-50 text-end">
                {format!("{:.2}", data.energy_per_kg)}
            </div>
            <div class="p-2 border-b group-hover:bg-amber-200 group-odd:bg-gray-50 text-end">
                {format!("{:.2}", data.protein_per_kg)}
            </div>
            <div class="p-2 border-b group-hover:bg-amber-200 group-odd:bg-gray-50 text-end">
                {format!("{:.2}", data.carbohydrate_per_kg)}
            </div>
            <div class="p-2 border-b group-hover:bg-amber-200 group-odd:bg-gray-50 text-end">
                {format!("{:.2}", data.fat_per_kg)}
            </div>
            <div class="px-2 border-b group-hover:bg-amber-200 text-end">
                {format!("{:.2}", data.weight)}
            </div>
        </div>
    }
}

#[component]
pub fn DaySummaryMonthAverageComponent(data: UserDaySummary) -> impl IntoView {
    view! {
        <div class="contents group">
            <div class="col-span-3 p-2 font-bold bg-gray-200">"Week Average"</div>
            <div class="p-2 font-bold bg-gray-200 text-end">
                {format!("{:.0}", data.energy)} "kcal"
            </div>
            <div class="p-2 font-bold bg-gray-200 text-end">
                {format!("{:.1}", data.protein)}
                <span class="ml-1 text-xs text-gray-400">
                    "(" {format!("{:.0}", data.protein_pct)} "%)"
                </span>
            </div>
            <div class="p-2 font-bold bg-gray-200 text-end">
                {format!("{:.1}", data.carbohydrate)}
                <span class="ml-1 text-xs text-gray-400">
                    "(" {format!("{:.0}", data.carbohydrate_pct)} "%)"
                </span>
            </div>
            <div class="p-2 font-bold bg-gray-200 text-end">
                {format!("{:.1}", data.fat)}
                <span class="ml-1 text-xs text-gray-400">
                    "(" {format!("{:.0}", data.fat_pct)} "%)"
                </span>
            </div>
            <div class="p-2 font-bold bg-gray-200 text-end">{format!("{:.1}", data.saturates)}</div>
            <div class="p-2 font-bold bg-gray-200 text-end">{format!("{:.1}", data.sugars)}</div>
            <div class="p-2 font-bold bg-gray-200 text-end">{format!("{:.1}", data.fibre)}</div>
            <div class="p-2 font-bold bg-gray-200 text-end">{format!("{:.2}", data.salt)}</div>
            <div class="p-2 font-bold bg-gray-200 text-end">
                {format!("{:.2}", data.energy_per_kg)}
            </div>
            <div class="p-2 font-bold bg-gray-200 text-end">
                {format!("{:.2}", data.protein_per_kg)}
            </div>
            <div class="p-2 font-bold bg-gray-200 text-end">
                {format!("{:.2}", data.carbohydrate_per_kg)}
            </div>
            <div class="p-2 font-bold bg-gray-200 text-end">
                {format!("{:.2}", data.fat_per_kg)}
            </div>
            <div class="p-2 font-bold bg-gray-200 text-end">{format!("{:.2}", data.weight)}</div>
        </div>
    }
}
