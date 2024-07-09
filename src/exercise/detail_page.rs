use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::model::{ExerciseBase, ExerciseQuery};
use crate::component::template::{ErrorComponent, LoadingComponent};
use crate::set::model::SetQuery;
use crate::util::datetime::DATE_FORMAT_SHORT;
use crate::workout::router::ExerciseDetailParam;

#[cfg(feature = "ssr")]
use crate::{
    auth::model::User, auth::service::get_request_user, error::Error, setup::get_pool,
    workout::model::WorkoutBase,
};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ExerciseQueryResponse {
    pub current: ExerciseQuery,
    pub previous: Option<ExerciseQuery>,
}

#[server(endpoint = "exercise-detail")]
pub async fn get_exercise_detail(exercise_id: Uuid) -> Result<ExerciseBase, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let exercise = ExerciseBase::get_by_id(&pool, exercise_id)
        .await?
        .ok_or(Error::NotFound)?;
    let workout = WorkoutBase::get_by_id(&pool, exercise.workout_id)
        .await?
        .ok_or(Error::NotFound)?;
    User::check_view_permission_by_user_id(&pool, &user, workout.user_id).await?;

    Ok(exercise)
}

#[server(endpoint = "exercise-detail-with-previous")]
pub async fn get_exercise_detail_with_previous(
    exercise_id: Uuid,
) -> Result<ExerciseQueryResponse, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let current = ExerciseQuery::try_get_with_sets(&pool, exercise_id)
        .await?
        .ok_or(Error::NotFound)?;
    let workout = WorkoutBase::get_by_id(&pool, current.workout_id)
        .await?
        .ok_or(Error::NotFound)?;
    workout.can_update(&user).await?;

    let previous_exercise_id =
        ExerciseBase::try_get_previous_exercise_id(&pool, exercise_id).await?;

    let previous = if let Some(id) = previous_exercise_id {
        Some(
            ExerciseQuery::try_get_with_sets(&pool, id)
                .await?
                .ok_or(Error::NotFound)?,
        )
    } else {
        None
    };

    Ok(ExerciseQueryResponse { current, previous })
}

#[component]
pub fn ExerciseDetailPage() -> impl IntoView {
    let params = use_params::<ExerciseDetailParam>();
    let id = move || params.with(|p| p.as_ref().map(|p| p.exercise_id).unwrap_or_default());

    let resource = Resource::new(id, get_exercise_detail_with_previous);

    let response = move || {
        resource.and_then(|data| {
            let current = &data.current;
            let previous = &data.previous;

            let previous_view = previous
                .as_ref()
                .map(|prev| view! { <ExerciseWithSetDetailComponent data=prev/> });

            view! {
                <ExerciseWithSetDetailComponent data=current/>
                <section class="mt-4">
                    <h2 class="mb-2 text-base font-bold">"Previous Exercise Detail"</h2>
                    {previous_view}
                </section>
            }
        })
    };

    view! {
        <Title text="Exercise Detail"/>
        <main class="p-4">
            <div class="grid grid-cols-4 gap-4 md:grid-cols-12">
                <div class="col-span-4">
                    <div class="p-4 bg-white border">
                        <h2 class="mb-2 text-base font-bold">"Exercise Detail"</h2>
                        <Transition fallback=LoadingComponent>
                            <ErrorBoundary fallback=|errors| {
                                view! { <ErrorComponent errors/> }
                            }>{response}</ErrorBoundary>
                        </Transition>
                    </div>
                </div>
            </div>
        </main>
    }
}

#[component]
pub fn ExerciseWithSetDetailComponent<'a>(data: &'a ExerciseQuery) -> impl IntoView {
    let movement_detail_url = format!("/exercises/{}", data.movement_slug);
    let workout_detail_url = format!(
        "/users/{}/workouts/{}/{}",
        data.username, data.date, data.workout_id
    );
    let workout_date = data.date.format(DATE_FORMAT_SHORT).to_string();

    let sets = &data.sets;
    let sets_view = sets
        .iter()
        .map(|inner| view! { <ExerciseWithSetDetailItem data=inner/> })
        .collect_view();
    view! {
        <header class="flex justify-between items-end mb-2">
            <h2 class="text-base font-bold">
                <a href=movement_detail_url class="hover:underline">
                    {data.order}
                    " "
                    {&data.movement_name}
                </a>
            </h2>
            <p class="text-gray-500">
                <a href=workout_detail_url class="hover:underline">
                    {workout_date}
                </a>
            </p>
        </header>

        <section>{sets_view}</section>
    }
}

#[component]
pub fn ExerciseWithSetDetailItem<'a>(data: &'a SetQuery) -> impl IntoView {
    let weight = format!("{:.2}kg", data.weight);

    view! {
        <div class="flex p-2 mb-1 bg-gray-200/50">
            <div class="flex-1">{data.order}</div>
            <div class="flex-1">{weight}</div>
            <div class="flex-1">{data.reps} " reps"</div>

        </div>
    }
}
