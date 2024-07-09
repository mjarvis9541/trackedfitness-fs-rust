use leptos::*;
use leptos_router::*;

use crate::component::template::{ErrorComponent, ListLoadingComponent, ListNotFoundComponent};
use crate::util::datetime::DATE_FORMAT_SHORT;
use crate::workout::layout::SidebarResource;
use crate::workout::model::WorkoutDayQuery;

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, setup::get_pool};

#[server(endpoint = "get-workout-sidebar")]
pub async fn get_workout_sidebar(
    username: String,
) -> Result<Vec<WorkoutDayQuery>, ServerFnError> {
    get_request_user()?;
    let pool = get_pool()?;
    let query = WorkoutDayQuery::get_all_by_username(&pool, &username).await?;
    Ok(query)
}

#[component]
pub fn WorkoutSidebar() -> impl IntoView {
    let resource = expect_context::<SidebarResource>();

    let response = move || {
        resource.and_then(|data| {
            if data.is_empty() {
                view! { <ListNotFoundComponent/> }
            } else {
                data.iter()
                    .map(|data| {
                        view! { <WorkoutAsideListItem data/> }
                    })
                    .collect_view()
            }
        })
    };

    view! {
        <h1 class="p-2 font-bold">"All Workouts"</h1>

        <section class="grid grid-cols-[3fr,1fr,1fr,1fr]">
            <div class="p-2 font-bold border-b">"Date"</div>
            <div class="p-2 font-bold border-b">"Exercises"</div>
            <div class="p-2 font-bold border-b">"Sets"</div>
            <div class="p-2 font-bold border-b">"Reps"</div>
            <Transition fallback=ListLoadingComponent>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors/> }
                }>{response}</ErrorBoundary>
            </Transition>
        </section>
    }
}

#[component]
pub fn WorkoutAsideListItem<'a>(data: &'a WorkoutDayQuery) -> impl IntoView {
    let date_display = data.workout_date.format(DATE_FORMAT_SHORT).to_string();
    view! {
        <div class="contents group">
            <div class="border-b group-hover:bg-amber-200 group-odd:bg-gray-50 truncate">
                <A
                    href=data.workout_date.to_string()
                    class="flex flex-1 p-2 aria-[current=page]:bg-amber-200"
                    exact=true
                >
                    {date_display}
                </A>
            </div>
            <div class="p-2 border-b group-hover:bg-amber-200 group-odd:bg-gray-50">
                {data.exercise_count}
            </div>
            <div class="p-2 border-b group-hover:bg-amber-200 group-odd:bg-gray-50">
                {data.set_count}
            </div>
            <div class="p-2 border-b group-hover:bg-amber-200 group-odd:bg-gray-50">
                {data.rep_count}
            </div>
        </div>
    }
}
