use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use uuid::Uuid;

use crate::component::button::SubmitButton;
use crate::component::template::{ErrorComponent, LoadingComponent};
use crate::exercise::detail_page::get_exercise_detail;
use crate::movement::select::{get_movement_select, MovementSelect};
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};
use crate::workout::router::ExerciseDetailParam;

#[cfg(feature = "ssr")]
use crate::{exercise::model::ExerciseBase, workout::model::WorkoutBase};

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, error::Error, setup::get_pool};

#[server(endpoint = "exercise-update")]
pub async fn exercise_update(
    exercise_id: Uuid,
    movement_id: Uuid,
    redirect_to: Option<String>,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let exercise = ExerciseBase::get_by_id(&pool, exercise_id)
        .await?
        .ok_or(Error::NotFound)?;
    let workout = WorkoutBase::get_by_id(&pool, exercise.workout_id)
        .await?
        .ok_or(Error::NotFound)?;
    workout.can_update(&user).await?;

    ExerciseBase::update(&pool, exercise.id, workout.id, movement_id, user.id).await?;

    if let Some(redirect_to) = redirect_to {
        leptos_axum::redirect(&redirect_to);
    }
    Ok(())
}

#[component]
pub fn ExerciseUpdatePage() -> impl IntoView {
    let params = use_params::<ExerciseDetailParam>();
    let id = move || params.with(|p| p.as_ref().map(|p| p.exercise_id).unwrap_or_default());

    let action = Action::<ExerciseUpdate, _>::server();
    let action_value = action.value();
    let action_error =
        move || extract_other_errors(action_value, &["non_field_errors", "movement_id"]);
    let non_field_errors = move || get_non_field_errors(action_value);

    let redirect_url = move || {
        params.with(|p| {
            p.as_ref()
                .map(|p| format!("/users/{}/workouts/{}/{}", p.username, p.date, p.workout_id))
                .unwrap_or_default()
        })
    };
    let resource = Resource::new(id, get_exercise_detail);

    let movement_resource = Resource::once(get_movement_select);
    provide_context(movement_resource);

    let response = move || {
        resource.and_then(|data| {
            let movement_id = data.movement_id.clone();
            let exercise_id = data.id.to_string();
            view! {
                <ActionForm action>
                    <input type="hidden" name="redirect_to" value=redirect_url/>
                    <input type="hidden" name="exercise_id" value=exercise_id/>
                    <MovementSelect name="movement_id" selected=movement_id/>
                    <SubmitButton loading=action.pending() label="Update Exercise"/>
                </ActionForm>
            }
        })
    };

    view! {
        <Title text="Edit Exercise"/>
        <main class="p-4 m-4 mx-auto max-w-md bg-white rounded border shadow-md">
            <h1 class="mb-2 text-base font-bold">"Edit Exercise"</h1>

            <div class="mb-4 text-red-500 font-bold">{action_error}</div>
            <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>

            <Transition fallback=LoadingComponent>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors/> }
                }>{response}</ErrorBoundary>
            </Transition>

        </main>
    }
}
