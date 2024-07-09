use leptos::*;
use leptos_router::*;

use chrono::prelude::*;

use crate::component::template::{ErrorComponent, LoadingSpinner};
use crate::util::param::{get_date, get_username};

use super::model::WorkoutDaySummary;

#[cfg(feature = "ssr")]
use crate::{
    auth::model::User,
    auth::service::get_request_user,
    setup::get_pool,
    util::datetime::{get_week_end, get_week_start},
};

#[server(endpoint = "workout-week-nav")]
pub async fn get_workout_week_nav(
    username: String,
    date: NaiveDate,
) -> Result<Vec<WorkoutDaySummary>, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    User::check_view_permission(&pool, &user, &username).await?;
    let start = get_week_start(date);
    let end = get_week_end(date);
    let query =
        WorkoutDaySummary::get_range_by_username_start_end(&pool, &username, start, end).await?;
    Ok(query)
}

#[component]
pub fn WorkoutWeekNavComponent() -> impl IntoView {
    let params = use_params_map();
    let username = move || get_username(&params);
    let date = move || get_date(&params);
    let resource = Resource::new(
        move || (username(), date()),
        |(username, date)| get_workout_week_nav(username, date),
    );
    let now = chrono::Utc::now().date_naive();

    let response = move || {
        resource.and_then(|res| {
            res.iter()
                .map(|data| {
                    let is_now = data.date == now;
                    let is_viewed = data.date == date();
                    view! { <WorkoutDaySummaryComponent data=data.clone() is_now is_viewed/> }
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
pub fn WorkoutDaySummaryComponent(
    data: WorkoutDaySummary,
    is_now: bool,
    is_viewed: bool,
) -> impl IntoView {
    let date_day_of_week = data.date.format("%a").to_string();
    let date_num = data.date.format("%d").to_string();
    let href = format!("/users/{}/workouts/{}", data.username, data.date);

    view! {
        <a
            class="block flex-1 p-2 hover:bg-amber-200"
            class=("bg-blue-500", is_now)
            class=("bg-blue-400", is_viewed)
            class=("text-gray-100", is_viewed || is_now)
            href=href
        >
            <h2 class="text-center">
                <span class="block md:inline-block">{date_day_of_week}</span>
                <span class="block md:inline-block md:ml-1">{date_num}</span>
            </h2>
            <div class="text-xs">
                <div>{data.total_workouts} " workouts"</div>
                <div>{data.total_exercises} " exercises"</div>
                <div>{data.total_sets} " sets | " {data.total_reps} " reps"</div>
            </div>
        </a>
    }
}
