use leptos::*;
use leptos_meta::*;

use crate::component::template::{ErrorComponent, LoadingComponent};
use crate::user_statistic::model::UserStatistic;

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, error::Error, setup::get_pool};

#[server(endpoint = "get-request-user-stats")]
pub async fn get_request_user_stats_detail() -> Result<UserStatistic, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    let query = UserStatistic::get_by_username(&pool, &user.username)
        .await?
        .ok_or(Error::NotFound)?;
    Ok(query)
}

#[component]
pub fn UserStatsDetailPage() -> impl IntoView {
    let resource = Resource::once(get_request_user_stats_detail);
    let response =
        move || resource.and_then(|data| view! { <UserStatDetailComponent data=data.clone()/> });
    view! {
        <Title text="Site Statistics"/>
        <main class="p-4 bg-white">
            <h1 class="mb-2 text-base font-bold">"Site Statistics"</h1>
            <p class="mb-4">"Site Statistics."</p>
            <div class="max-w-md">
                <Transition fallback=LoadingComponent>
                    <ErrorBoundary fallback=|errors| {
                        view! { <ErrorComponent errors/> }
                    }>{response}</ErrorBoundary>
                </Transition>
            </div>
        </main>
    }
}

#[component]
pub fn UserStatDetailComponent(data: UserStatistic) -> impl IntoView {
    view! {
        <table class="w-full border-collapse">
            <tbody>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Followers"</th>
                    <td class="p-2 w-1/2 text-right border">{data.follower_count}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Following"</th>
                    <td class="p-2 w-1/2 text-right border">{data.following_count}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Diet Log Entries"</th>
                    <td class="p-2 w-1/2 text-right border">{data.diet_count}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Diet Day Entries"</th>
                    <td class="p-2 w-1/2 text-right border">{data.diet_day_log_count}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Diet Target Entries"</th>
                    <td class="p-2 w-1/2 text-right border">{data.diet_target_count}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Progress Log Entries"</th>
                    <td class="p-2 w-1/2 text-right border">{data.progress_count}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Workouts Logged"</th>
                    <td class="p-2 w-1/2 text-right border">{data.workout_count}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Exercises Logged"</th>
                    <td class="p-2 w-1/2 text-right border">{data.exercise_count}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Sets Logged"</th>
                    <td class="p-2 w-1/2 text-right border">{data.set_count}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Reps Logged"</th>
                    <td class="p-2 w-1/2 text-right border">{data.rep_count}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Food Created"</th>
                    <td class="p-2 w-1/2 text-right border">{data.food_created_count}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Brands Created"</th>
                    <td class="p-2 w-1/2 text-right border">{data.brand_created_count}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Meal of Day Created"</th>
                    <td class="p-2 w-1/2 text-right border">{data.meal_of_day_created_count}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Meals Created"</th>
                    <td class="p-2 w-1/2 text-right border">{data.meal_created_count}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Meal Food Created"</th>
                    <td class="p-2 w-1/2 text-right border">{data.meal_food_created_count}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Exercises Created"</th>
                    <td class="p-2 w-1/2 text-right border">{data.movement_created_count}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Muscle Groups Created"</th>
                    <td class="p-2 w-1/2 text-right border">{data.muscle_group_created_count}</td>
                </tr>
            </tbody>
        </table>
    }
}
