use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use chrono::prelude::*;
use uuid::Uuid;

use super::model::WorkoutWeek;
use crate::component::template::{ErrorComponent, LoadingComponent};
use crate::exercise::model::ExerciseQuery;
use crate::set::model::SetQuery;
use crate::util::datetime::DATE_FORMAT_LONG;
use crate::util::param::{get_date, get_username};
use crate::workout::model::WorkoutQuery;

#[cfg(feature = "ssr")]
use crate::{auth::model::User, auth::service::get_request_user, setup::get_pool};

#[server]
pub async fn get_workout_week(
    username: String,
    date: NaiveDate,
) -> Result<Vec<WorkoutWeek>, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    User::check_view_permission(&pool, &user, &username).await?;
    let query = WorkoutWeek::all(&pool, &username, date).await?;
    Ok(query)
}

#[component]
pub fn WorkoutWeekTopBar() -> impl IntoView {
    let params = use_params_map();
    let username = move || get_username(&params);
    let date = move || get_date(&params);

    let resource = Resource::new(
        move || (username(), date()),
        |(username, date)| get_workout_week(username, date),
    );
    let response = move || {
        resource.and_then(|res| {
            res.iter()
                .map(|data| view! { <DayOfWeekListItem data/> })
                .collect_view()
        })
    };
    view! {
        <Title text="Workout Week"/>
        <main>
            <div class="grid grid-cols-1 gap-2 lg:grid-cols-7">
                <Transition fallback=LoadingComponent>
                    <ErrorBoundary fallback=|errors| {
                        view! { <ErrorComponent errors=errors/> }
                    }>{response}</ErrorBoundary>
                </Transition>
            </div>
        </main>
    }
}

#[component]
pub fn DayOfWeekListItem<'a>(data: &'a WorkoutWeek) -> impl IntoView {
    let workout_day_href = format!("/users/{}/workouts/{}", data.username, data.date);
    let date_title = data.date.format(DATE_FORMAT_LONG).to_string();

    let username = &data.username;
    let date = &data.date;

    let workouts = &data.workouts;
    let workouts_view = workouts
        .iter()
        .map(|inner| {
            view! { <WorkoutListItem data=inner username date/> }
        })
        .collect_view();

    view! {
        <div class="bg-white">
            <a
                href=workout_day_href
                class="flex p-2 font-bold bg-gray-200 duration-300 hover:bg-amber-200"
            >
                {date_title}
            </a>
            {workouts_view}
        </div>
    }
}

#[component]
pub fn WorkoutListItem<'a>(
    data: &'a WorkoutQuery,
    username: &'a String,
    date: &'a NaiveDate,
) -> impl IntoView {
    let workout_id = &data.workout_id;
    let workout_detail_href = format!("/users/{}/workouts/{}/{}", username, date, workout_id);

    let created_at = data.created_at.format("%r").to_string();

    let exercises = &data.exercises;
    let exercises_view = exercises
        .iter()
        .map(|inner| {
            view! { <ExerciseListItem data=inner username date workout_id/> }
        })
        .collect_view();

    view! {
        <a href=workout_detail_href class="block p-2 bg-gray-100 hover:bg-amber-200">
            <h1 class="font-bold">"Workout"</h1>
            <p class="text-gray-500">{created_at}</p>
        </a>
        <div>{exercises_view}</div>
    }
}
#[component]
pub fn ExerciseListItem<'a>(
    data: &'a ExerciseQuery,
    username: &'a String,
    date: &'a NaiveDate,
    workout_id: &'a Uuid,
) -> impl IntoView {
    let exercise_id = &data.id;
    let exercise_detail_href = format!(
        "/users/{}/workouts/{}/{}/{}",
        username, date, workout_id, data.id
    );

    let sets = &data.sets;
    let sets_view = sets
        .iter()
        .map(|inner| {
            view! { <SetListItem data=inner username date workout_id exercise_id/> }
        })
        .collect_view();

    view! {
        <a href=exercise_detail_href class="block p-2 bg-gray-100 hover:bg-amber-200 truncate">
            <h2 class="font-bold">{data.order} ". " {&data.movement_name}</h2>
            <p class="text-gray-500">{&data.muscle_group_name}</p>
        </a>
        <div class="flex px-2">
            <div class="flex-1">"Set"</div>
            <div class="flex-1 text-center">"Weight"</div>
            <div class="flex-1 text-center">"Reps"</div>
        </div>
        <div class="">{sets_view}</div>
    }
}

#[component]
pub fn SetListItem<'a>(
    data: &'a SetQuery,
    username: &'a String,
    date: &'a NaiveDate,
    workout_id: &'a Uuid,
    exercise_id: &'a Uuid,
) -> impl IntoView {
    let detail_href = format!(
        "/users/{}/workouts/{}/{}/{}/{}",
        username, date, workout_id, exercise_id, data.id
    );
    view! {
        <a class="flex px-2 hover:bg-amber-200" href=detail_href>
            <div class="flex-1">{data.order}</div>
            <div class="flex-1 text-center">{format!("{:.0}", data.weight)}</div>
            <div class="flex-1 text-center">{data.reps}</div>
        </a>
    }
}
