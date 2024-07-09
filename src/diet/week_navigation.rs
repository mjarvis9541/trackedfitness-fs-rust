use leptos::*;
use leptos_router::*;

use chrono::prelude::*;

use crate::component::template::{ErrorComponent, LoadingSpinner};
use crate::util::param::{get_date, get_username};

use super::model::DietDaySummary;

#[cfg(feature = "ssr")]
use crate::{auth::model::User, auth::service::get_request_user, setup::get_pool};

#[server(endpoint = "user-diet-day-week-navigation")]
pub async fn get_diet_week_nav(
    username: String,
    date: NaiveDate,
) -> Result<Vec<DietDaySummary>, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    User::check_view_permission(&pool, &user, &username).await?;
    let query = DietDaySummary::get_by_username_date(&pool, &username, date).await?;
    Ok(query)
}

#[component]
pub fn DietWeekNavComponent() -> impl IntoView {
    let params = use_params_map();
    let username = move || get_username(&params);
    let date = move || get_date(&params);
    let resource = Resource::new(
        move || (username(), date()),
        |(username, date)| get_diet_week_nav(username, date),
    );
    let now = chrono::Utc::now().date_naive();

    let response = move || {
        resource.and_then(|res| {
            res.iter()
                .map(|data| {
                    let is_now = data.date == now;
                    let is_viewed = data.date == date();
                    view! { <DietDaySummaryComponent data=data.clone() is_now is_viewed/> }
                })
                .collect_view()
        })
    };

    view! {
        <div class="grid grid-cols-7">
            <Transition fallback=LoadingSpinner>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors/> }
                }>{response}</ErrorBoundary>
            </Transition>
        </div>
    }
}

#[component]
pub fn DietDaySummaryComponent(
    data: DietDaySummary,
    is_now: bool,
    is_viewed: bool,
) -> impl IntoView {
    let date_day_of_week = data.date.format("%a").to_string();
    let date_num = data.date.format("%d").to_string();
    let href = format!("/users/{}/diet/{}", data.username, data.date);

    view! {
        <a
            class="block flex-1 p-2 text-center hover:text-gray-900 hover:bg-amber-200"
            class=("bg-blue-500", is_now)
            class=("bg-blue-400", is_viewed)
            class=("text-gray-100", is_viewed || is_now)
            href=href
        >
            <h2>
                <span class="block md:inline-block">{date_day_of_week}</span>
                <span class="block md:inline-block md:ml-1">{date_num}</span>
            </h2>
            <div class="text-xs">
                <span class="block md:inline-block">{format!("{:.0}", data.energy)}</span>
                <span class="block md:inline-block md:ml-1">"kcal"</span>
            </div>
            <div class="hidden text-xs md:block">
                {format!("{:.0}g pro | ", data.protein)}
                {format!("{:.0}g carbs | ", data.carbohydrate)} {format!("{:.0}g fat", data.fat)}
            </div>
        </a>
    }
}
