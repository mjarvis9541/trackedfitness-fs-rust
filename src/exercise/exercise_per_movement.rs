use leptos::*;
use leptos_router::*;

use crate::component::template::{ErrorComponent, ListNotFoundComponent, LoadingComponent};
use crate::exercise::model::ExerciseQuery;
use crate::set::model::SetQuery;
use crate::util::datetime::DATE_FORMAT_SHORT;
use crate::util::param::get_slug;

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, setup::get_pool};

#[server(endpoint = "exercise-per-movement-detail")]
pub async fn get_all_exercise_per_movement(
    movement_slug: String,
) -> Result<Vec<ExerciseQuery>, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let query = ExerciseQuery::get_all_with_sets_for_user_by_movement(
        &pool,
        &user.username,
        &movement_slug,
    )
    .await?;
    Ok(query)
}

#[component]
pub fn ExerciseMovementComponent() -> impl IntoView {
    let params = use_params_map();
    let slug = move || get_slug(&params);
    let resource = Resource::new(slug, get_all_exercise_per_movement);
    let response = move || {
        resource.and_then(|res| {
            if res.is_empty() {
                view! { <ListNotFoundComponent/> }
            } else {
                res.iter()
                    .map(|data| {
                        view! { <ExerciseMovementListItem data/> }
                    })
                    .collect_view()
            }
        })
    };

    view! {
        <h1 class="mb-4 text-xl font-bold">"Exercises Performed"</h1>

        <Transition fallback=LoadingComponent>
            <ErrorBoundary fallback=|errors| {
                view! { <ErrorComponent errors/> }
            }>{response}</ErrorBoundary>
        </Transition>
    }
}

#[component]
pub fn ExerciseMovementListItem<'a>(data: &'a ExerciseQuery) -> impl IntoView {
    let sets = &data.sets;
    let sets_view = sets
        .iter()
        .map(|set| {
            view! { <ExerciseMovementSetListItem data=set/> }
        })
        .collect_view();

    let workout_date_href = format!("/users/{}/workouts/{}", data.username, data.date);
    let workout_detail_href = format!(
        "/users/{}/workouts/{}/{}",
        data.username, data.date, data.workout_id
    );
    let exercise_detail_href = format!(
        "/users/{}/workouts/{}/{}/{}",
        data.username, data.date, data.workout_id, data.id
    );
    view! {
        <div class="p-4 mb-4 border">
            <header class="flex justify-between items-start mb-1">
                <div>
                    <h2 class="text-base font-bold">"Workout"</h2>
                </div>
                <div>
                    <a href=workout_date_href class="block hover:underline">
                        {data.date.format(DATE_FORMAT_SHORT).to_string()}
                    </a>
                </div>
            </header>
            <section class="mb-2">
                <a href=workout_detail_href class="block mb-1 text-gray-400 hover:underline">
                    "Workout: "
                    {data.workout_id.to_string()}
                </a>
                <a href=exercise_detail_href class="block mb-1 text-gray-400 hover:underline">
                    "Exercise: "
                    {data.id.to_string()}
                </a>
            </section>

            <table class="w-full border border-collapse">
                <thead>
                    <tr>
                        <th class="p-2 w-1/3 border text-start">"Set"</th>
                        <th class="p-2 w-1/3 border text-start">"Weight"</th>
                        <th class="p-2 w-1/3 border text-start">"Reps"</th>
                    </tr>
                </thead>
                <tbody>{sets_view}</tbody>
            </table>
        </div>
    }
}

#[component]
pub fn ExerciseMovementSetListItem<'a>(data: &'a SetQuery) -> impl IntoView {
    let weight = format!("{:.2}kg", data.weight);
    view! {
        <tr>
            <th class="p-2 border text-start">{data.order}</th>
            <td class="p-2 border">{weight}</td>
            <td class="p-2 border">{data.reps}</td>
        </tr>
    }
}
