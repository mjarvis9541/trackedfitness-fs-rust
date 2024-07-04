use leptos::*;
use leptos_router::*;

use chrono::prelude::*;
use rust_decimal::Decimal;
use uuid::Uuid;

use super::component::WorkoutDayHeader;
use crate::component::badge::Badge;
use crate::component::date_navigation::DateNavigation;
use crate::component::icon::{IconEditA, IconFilePlus};
use crate::component::link::Link;
use crate::component::template::{ErrorComponent, ListNotFoundComponent, LoadingComponent};
use crate::exercise::create_page::ExerciseCreate;
use crate::exercise::delete_page::{ExerciseDelete, ExerciseDeleteForm};
use crate::exercise::model::ExerciseQueryWithPrevious;
use crate::set::create_form::{SetCreate, SetCreateForm};
use crate::set::delete_page::{SetDelete, SetDeleteForm};
use crate::set::model::SetQueryWithPrevious;
use crate::set::update_page::{SetRowUpdateForm, SetUpdate};
use crate::util::param::{get_date, get_username};
use crate::workout::create_page::WorkoutCreate;
use crate::workout::delete_page::{WorkoutDelete, WorkoutDeleteForm};
use crate::workout::model::WorkoutQueryWithPrevious;

#[cfg(feature = "ssr")]
use crate::{
    auth::model::User, auth::service::get_request_user, setup::get_pool,
    workout::model::WorkoutDetail,
};

#[server(endpoint = "user-workout-day")]
pub async fn get_workout_day(
    username: String,
    date: NaiveDate,
) -> Result<Vec<WorkoutQueryWithPrevious>, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    User::check_view_permission(&pool, &user, &username).await?;
    let query = WorkoutQueryWithPrevious::get(&pool, &username, date).await?;

    let q2 = WorkoutDetail::aggregate_workout_day_data(&pool, &username, date).await?;

    let json = serde_json::to_string_pretty(&query).unwrap();
    let json2 = serde_json::to_string_pretty(&q2).unwrap();

    // Print the JSON string
    println!("{}", json);
    println!("{}", json2);
    Ok(query)
}

#[component]
pub fn WorkoutDayPage() -> impl IntoView {
    let action_workout_create =
        expect_context::<Action<WorkoutCreate, Result<(), ServerFnError>>>();
    let action_workout_delete =
        expect_context::<Action<WorkoutDelete, Result<(), ServerFnError>>>();
    let action_exercise_create =
        expect_context::<Action<ExerciseCreate, Result<(), ServerFnError>>>();
    let action_exercise_delete =
        expect_context::<Action<ExerciseDelete, Result<(), ServerFnError>>>();
    let action_set_create = expect_context::<Action<SetCreate, Result<(), ServerFnError>>>();
    let action_set_update = expect_context::<Action<SetUpdate, Result<(), ServerFnError>>>();
    let action_set_delete = expect_context::<Action<SetDelete, Result<(), ServerFnError>>>();

    let params = use_params_map();
    let username = move || get_username(&params);
    let date = move || get_date(&params);

    let resource = Resource::new(
        move || {
            (
                username(),
                date(),
                action_exercise_create.version().get(),
                action_exercise_delete.version().get(),
                action_workout_create.version().get(),
                action_workout_delete.version().get(),
                action_set_create.version().get(),
                action_set_update.version().get(),
                action_set_delete.version().get(),
            )
        },
        |(username, date, ..)| get_workout_day(username, date),
    );
    let response = move || {
        resource.and_then(|data| {
            if data.is_empty() {
                view! { <ListNotFoundComponent/> }
            } else {
                data.iter()
                    .map(|data| {
                        view! { <WorkoutListItemComponent data=data.clone()/> }
                    })
                    .collect_view()
            }
        })
    };

    view! {
        <main class="p-4">
            <DateNavigation/>
            <WorkoutDayHeader title="Workouts"/>

            <Transition fallback=LoadingComponent>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors/> }
                }>{response}</ErrorBoundary>
            </Transition>
        </main>
    }
}

#[component]
pub fn WorkoutListItemComponent(data: WorkoutQueryWithPrevious) -> impl IntoView {
    let subtitle = data.format_date();
    let add_exercise_url = data.get_add_exercise_url();
    let add_workout_url = data.get_add_workout_plan_url();
    let detail_url = data.get_workout_detail_url();

    let data = data.clone();
    let exercises = data.exercises;

    let exercises_view = exercises
        .into_iter()
        .map(|exercise_data| {
            view! {
                <ExerciseListItemComponent
                    data=exercise_data
                    username=data.username.clone()
                    date=data.workout_date
                    workout_id=data.workout_id
                />
            }
        })
        .collect_view();

    let exercise_count = data.exercise_count;
    let set_count = data.set_count;
    let rep_count = data.rep_count;
    let workout_id = data.workout_id.to_string();

    view! {
        <header class="flex flex-wrap gap-4 items-start p-2 mb-1 bg-gray-300">
            <a class="block flex-1" href=detail_url>
                <h2 class="text-base font-bold">"Workout"</h2>
                <p class="mb-2 text-xs text-gray-500">{subtitle}</p>
            </a>

            <section class="flex gap-2">
                <Badge label="exercises" value=exercise_count/>
                <Badge label="sets" value=set_count/>
                <Badge label="reps" value=rep_count/>
            </section>

            <section class="flex gap-x-2">
                <Link href=add_exercise_url text="Add Exercise">
                    <IconFilePlus/>
                </Link>
                <Link href=add_workout_url text="Add Workout Plan">
                    <IconFilePlus/>
                </Link>
                <WorkoutDeleteForm id=workout_id/>
            </section>
        </header>

        {exercises_view}

        <div class="h-1"></div>
    }
}

#[component]
pub fn ExerciseListItemComponent(
    data: ExerciseQueryWithPrevious,
    username: String,
    date: NaiveDate,
    workout_id: Uuid,
) -> impl IntoView {
    let title = data.get_title();
    let next_set_reps = data.get_last_set_reps();
    let next_set_weight = data.get_last_set_weight();
    let next_set_order = data.get_next_set_order();

    let exercise_id = data.exercise_id;

    let exercise_detail_href = {
        format!(
            "/users/{}/workouts/{}/{}/{}",
            username, date, workout_id, exercise_id,
        )
    };

    let sets = data.sets;
    let sets_view = sets.into_iter()
        .map(|inner| view! { <SetListItemComponent data=inner username=username.clone() date workout_id exercise_id/> })
        .collect_view();

    let exercise_id_str = exercise_id.to_string();
    let exercise_id_str_b = exercise_id.to_string();

    view! {
        <header class="flex flex-wrap gap-4 items-start p-2 mb-1 bg-gray-200">

            <a class="flex-1 whitespace-nowrap" href=exercise_detail_href>
                <h2 class="font-bold">{title}</h2>
                <p class="mb-2 text-xs text-gray-500">{data.muscle_group_name}</p>
            </a>

            <section class="flex flex-wrap gap-x-2">
                <Badge label="sets" value=data.set_count/>
                <Badge label="reps" value=data.rep_count/>
            </section>

            <section class="flex gap-x-2">
                <SetCreateForm
                    exercise_id=exercise_id_str
                    order=next_set_order
                    weight=next_set_weight
                    reps=next_set_reps
                />
                <ExerciseDeleteForm id=exercise_id_str_b/>
            </section>

        </header>

        {sets_view}
    }
}

#[component]
pub fn SetListItemComponent(
    data: SetQueryWithPrevious,
    username: String,
    date: NaiveDate,
    workout_id: Uuid,
    exercise_id: Uuid,
) -> impl IntoView {
    let set_detail_href = {
        format!(
            "/users/{}/workouts/{}/{}/{}/{}",
            username, date, workout_id, exercise_id, data.set_id,
        )
    };

    let previous_set_view = match (
        data.previous_weight,
        data.previous_reps,
        data.previous_workout_date,
    ) {
        (Some(weight), Some(reps), Some(workout_date)) => {
            view! {
                <SetListItemComponentPrevious
                    weight=weight
                    reps=reps
                    username=username.clone()
                    workout_date=workout_date
                />
            }
        }
        _ => {
            view! { <SetListItemComponentNoPrevious/> }
        }
    };

    let set_id = data.set_id.to_string();
    view! {
        <div class="flex flex-wrap gap-4 items-center p-1 mb-1 bg-gray-200 hover:bg-amber-200">
            <section class="flex gap-4">
                <div>{previous_set_view}</div>
                <div class="flex gap-4 items-center">
                    <SetRowUpdateForm data/>
                </div>
            </section>
            <section class="flex gap-2 ml-auto">
                <Link href=set_detail_href>
                    <IconEditA/>
                </Link>
                <SetDeleteForm id=set_id/>
            </section>
        </div>
    }
}

#[component]
fn SetListItemComponentPrevious(
    workout_date: NaiveDate,
    username: String,
    weight: Decimal,
    reps: i32,
) -> impl IntoView {
    let previous_date = workout_date.format("%a %d %b").to_string();

    view! {
        <A
            class="block px-2 hover:bg-amber-300 min-w-40"
            href=format!("/users/{}/workouts/{}", username, workout_date)
        >
            <div class="text-xs text-gray-500">{previous_date}</div>
            <div class="text-gray-700">{format!("{:.2}", weight)} "kg x " {reps}</div>
        </A>
    }
}

#[component]
fn SetListItemComponentNoPrevious() -> impl IntoView {
    view! {
        <div class="px-2 min-w-40">
            <div class="text-xs text-gray-500">"Previous"</div>
            <div class="text-gray-500">"No previous set"</div>
        </div>
    }
}
