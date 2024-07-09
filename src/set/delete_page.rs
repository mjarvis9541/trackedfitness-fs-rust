use leptos::*;
use leptos_router::*;

use uuid::Uuid;

use crate::component::button::{Button, ButtonVariant, SubmitButton};
use crate::component::icon::IconTrash;
use crate::component::template::{DetailPageTemplate, ErrorComponent, LoadingComponent};
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};
use crate::workout::router::SetDetailParam;

use super::detail_page::get_set_detail;

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user, error::Error, exercise::model::ExerciseBase,
    set::model::SetModel, setup::get_pool, workout::model::WorkoutBase,
};

#[server(endpoint = "set-delete")]
pub async fn set_delete(set_id: Uuid, redirect_to: Option<String>) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let set = SetModel::get_by_id(&pool, set_id)
        .await?
        .ok_or(Error::NotFound)?;
    let exercise = ExerciseBase::get_by_id(&pool, set.exercise_id)
        .await?
        .ok_or(Error::NotFound)?;
    let workout = WorkoutBase::get_by_id(&pool, exercise.workout_id)
        .await?
        .ok_or(Error::NotFound)?;
    workout.can_update(&user).await?;

    SetModel::delete(&pool, set_id).await?;

    if let Some(redirect_to) = redirect_to {
        leptos_axum::redirect(&redirect_to);
    }
    Ok(())
}

#[component]
pub fn SetDeleteForm(id: String) -> impl IntoView {
    let action = expect_context::<Action<SetDelete, Result<(), ServerFnError>>>();
    view! {
        <ActionForm action class="contents">
            <input type="hidden" name="set_id" value=id/>
            <Button variant=ButtonVariant::Danger>
                <IconTrash/>
            </Button>
        </ActionForm>
    }
}

#[component]
pub fn SetDeletePage() -> impl IntoView {
    let params = use_params::<SetDetailParam>();
    let set_id = move || params.with(|q| q.as_ref().map_or_else(|_| Uuid::default(), |q| q.set_id));

    let action = Action::<SetDelete, _>::server();
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || extract_other_errors(action_value, &["name", "ordering"]);
    let non_field_errors = move || get_non_field_errors(action_value);

    let redirect_to_url = move || {
        params.with(|p| {
            p.as_ref().map_or_else(
                |_| String::from("/"),
                |p| format!("/users/{}/workouts/{}", p.username, p.date),
            )
        })
    };

    let resource = Resource::new(set_id, get_set_detail);
    let response = move || {
        resource.and_then(|data| {
            let id = data.id.to_string();
            view! {
                <p class="mb-4">"Are you sure you wish to delete this set?"</p>
                <p class="mb-4">"Ths action cannot be undone."</p>
                <ActionForm action>
                    <input type="hidden" name="set_id" value=id/>
                    <input type="hidden" name="redirect_to" value=redirect_to_url/>
                    <SubmitButton loading=action_loading label="Delete Set"/>
                </ActionForm>
            }
        })
    };

    view! {
        <DetailPageTemplate title="Delete Set">
            <div class="mb-4 text-red-500 font-bold">{action_error}</div>
            <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>
            <Transition fallback=LoadingComponent>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors/> }
                }>{response}</ErrorBoundary>
            </Transition>
        </DetailPageTemplate>
    }
}
