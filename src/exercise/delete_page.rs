use leptos::*;
use leptos_router::*;

use uuid::Uuid;

use super::detail_page::get_exercise_detail;
use crate::component::button::{Button, ButtonVariant, SubmitButton};
use crate::component::icon::IconTrash;
use crate::component::template::{DeletePageWrapper, ErrorComponent, LoadingComponent};
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};
use crate::workout::router::ExerciseDetailParam;

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user, error::Error, exercise::model::ExerciseModel, setup::get_pool,
    workout::model::WorkoutBase,
};

#[server(endpoint = "exercise-delete")]
pub async fn exercise_delete(
    exercise_id: Uuid,
    redirect_to: Option<String>,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let exercise = ExerciseModel::get_by_id(&pool, exercise_id)
        .await?
        .ok_or(Error::NotFound)?;
    let workout = WorkoutBase::get_by_id(&pool, exercise.workout_id)
        .await?
        .ok_or(Error::NotFound)?;
    workout.can_delete(&user).await?;

    ExerciseModel::delete(&pool, exercise.id).await?;
    if let Some(redirect_to) = redirect_to {
        leptos_axum::redirect(&redirect_to);
    }
    Ok(())
}

#[component]
pub fn ExerciseDeleteForm(id: String) -> impl IntoView {
    let action = expect_context::<Action<ExerciseDelete, Result<(), ServerFnError>>>();
    view! {
        <ActionForm action class="contents">
            <input type="hidden" name="exercise_id" value=id/>
            <Button variant=ButtonVariant::Danger>
                <IconTrash/>
            </Button>
        </ActionForm>
    }
}

#[component]
pub fn ExerciseDeletePage() -> impl IntoView {
    let action = Action::<ExerciseDelete, _>::server();
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || extract_other_errors(action_value, &["name"]);
    let non_field_errors = move || get_non_field_errors(action_value);

    let params = use_params::<ExerciseDetailParam>();
    let exercise_id =
        move || params.with(|p| p.as_ref().map(|p| p.exercise_id).unwrap_or_default());

    let redirect_url = move || {
        params.with(|p| {
            p.as_ref()
                .map(|p| format!("/users/{}/workouts/{}/{}", p.username, p.date, p.workout_id))
                .unwrap_or_default()
        })
    };
    let resource = Resource::new(exercise_id, get_exercise_detail);

    let response = move || {
        resource.and_then(|data| {
            let exercise_id = data.id.to_string();
            view! {
                <ActionForm action>
                    <input type="hidden" name="redirect_to" value=redirect_url/>
                    <input type="hidden" name="exercise_id" value=exercise_id/>
                    <SubmitButton loading=action_loading label="Delete Exercise"/>
                </ActionForm>
            }
        })
    };

    view! {
        <DeletePageWrapper title="Exercise">
            <div class="mb-4 text-red-500 font-bold">{action_error}</div>
            <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>
            <Transition fallback=LoadingComponent>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors/> }
                }>{response}</ErrorBoundary>
            </Transition>
        </DeletePageWrapper>
    }
}
