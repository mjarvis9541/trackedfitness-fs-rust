use std::collections::HashSet;

use chrono::NaiveDate;
use leptos::*;
use leptos_router::*;

use crate::component::bulk_delete_date::BulkDeleteDateRangeForm;
use crate::component::template::{AutoListHeader, ErrorComponent, ListLoadingComponent};
use crate::util::param::{get_date, get_username};

use super::component::DaySummaryListItem;
use super::model::UserWeekSummary;

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user,
    setup::get_pool,
    summary::model::UserDaySummary,
    util::datetime::{get_week_end, get_week_start},
};

#[server]
pub async fn get_diet_week_summary(
    username: String,
    date: NaiveDate,
) -> Result<UserWeekSummary, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    let start = get_week_start(date);
    let end = get_week_end(date);
    let query = UserDaySummary::get_diet_range(&pool, &username, start, end).await?;
    dbg!(&query);

    let total_days = query.len() as i32;
    let week_avg = UserDaySummary::calculate_averages(&query, total_days)?;

    let complete_data = UserDaySummary::fill_missing_days_with_previous_with_default(
        query,
        start,
        end,
        &user.username,
    )?;
    Ok(UserWeekSummary {
        day_total_vec: complete_data,
        period_avg: week_avg,
    })
}

#[component]
pub fn DietWeekSummaryComponent() -> impl IntoView {
    let params = use_params_map();
    let username = move || get_username(&params);
    let date = move || get_date(&params);
    let action_bulk_delete = Action::server();

    let week_resource = Resource::new(
        move || (username(), date(), action_bulk_delete.version().get()),
        |(username, date, ..)| get_diet_week_summary(username, date),
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
                .map(|day_item| view! { <DaySummaryListItem data=day_item.clone() checked_items/> })
                .collect_view();

            view! {
                {day_view}
                <DaySummaryListItem data=data.period_avg.clone() total_row=true/>
            }
        })
    };
    view! {
        <div class="grid grid-cols-checkbox-16">
            <div class="col-span-full">
                <h2 class="text-base font-bold">"Diet Week Summary"</h2>
            </div>
            <AutoListHeader all_items checked_items align_right=true>
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

            <div class="mt-4 col-span-full">
                <BulkDeleteDateRangeForm
                    table="food_log"
                    username=Signal::derive(username)
                    action=action_bulk_delete
                    checked_items
                />
            </div>

        </div>
    }
}
