use std::collections::HashSet;

use chrono::NaiveDate;
use leptos::*;
use leptos_router::*;

use crate::component::bulk_delete_date::BulkDeleteDateRangeForm;
use crate::component::template::{ErrorComponent, ListLoadingComponent};
use crate::diet_target::update_or_create_form::DietTargetBulkUpdateOrCreateForm;
use crate::util::param::{get_date, get_username};

use super::component::{DaySummaryListItem, SummaryListHeader};
use super::model::{UserWeekSummary, Variant};

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user,
    setup::get_pool,
    summary::model::UserDaySummary,
    util::datetime::{get_week_end, get_week_start},
};

#[server]
pub async fn get_diet_target_week_summary(
    username: String,
    date: NaiveDate,
) -> Result<UserWeekSummary, ServerFnError> {
    let _user = get_request_user()?;
    let pool = get_pool()?;
    let start = get_week_start(date);
    let end = get_week_end(date);
    let mut query = UserDaySummary::get_target_range(&pool, &username, start, end).await?;

    let latest = if query.first().map_or(true, |first| first.date != start) {
        UserDaySummary::get_target_latest(&pool, &username, start).await?
    } else {
        None
    };

    if let Some(mut latest) = latest {
        if query.is_empty() {
            latest.date = start;
            latest.actual = false;
            query.push(latest);
        } else {
            // Ensure all dates before the first available date are filled with the latest summary
            let mut filled_days = Vec::new();
            let mut current_date = start;
            while current_date < query.first().unwrap().date {
                let mut latest = latest.clone();
                latest.date = current_date;
                latest.actual = false;
                filled_days.push(latest);
                current_date = current_date.succ_opt().unwrap_or_default();
            }
            query.splice(0..0, filled_days); // Insert the filled days at the beginning
        }
    }
    let complete_day_totals = UserDaySummary::fill_missing_days_with_previous(query, start, end)?;

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

    let action_bulk_delete = Action::server();
    let action_bulk_update = Action::server();

    let week_resource = Resource::new(
        move || {
            (
                username(),
                date(),
                action_bulk_delete.version().get(),
                action_bulk_update.version().get(),
            )
        },
        |(username, date, ..)| get_diet_target_week_summary(username, date),
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
                .map(|day_item| view! { <DaySummaryListItem data=day_item.clone() checked_items variant=Variant::DietTarget/> })
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
                <h2 class="text-base font-bold">"Diet Target Week Summary"</h2>
            </div>
            <SummaryListHeader all_items checked_items>
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
            </SummaryListHeader>
            <Transition fallback=ListLoadingComponent>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors/> }
                }>{week_response}</ErrorBoundary>
            </Transition>

            <div class="mt-4 col-span-full">
                <BulkDeleteDateRangeForm
                    table="diet_target"
                    username=Signal::derive(username)
                    action=action_bulk_delete
                    checked_items=checked_items
                />
            </div>

            <div class="mt-4 col-span-full">
                <div class="p-4 max-w-md bg-white border">
                    <h2 class="mb-4 text-base font-bold">"Update Selected Diet Targets"</h2>
                    <DietTargetBulkUpdateOrCreateForm
                        checked_items=checked_items
                        action=action_bulk_update
                    />
                </div>
            </div>

        </div>
    }
}
