use leptos::*;
use leptos_router::*;

use chrono::prelude::*;
use uuid::Uuid;

use crate::component::button::SubmitButton;
use crate::component::input::TextInput;
use crate::component::template::{DetailPageTemplate, ErrorComponent, LoadingComponent};
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

use super::detail_page::get_workout_detail;
use super::router::WorkoutDetailParam;

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user, error::Error, setup::get_pool, workout::model::WorkoutBase,
};

#[server(endpoint = "workout-update")]
pub async fn workout_update(
    workout_id: Uuid,
    date: NaiveDate,
    redirect_to: Option<String>,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let workout = WorkoutBase::get_by_id(&pool, workout_id)
        .await?
        .ok_or(Error::NotFound)?;
    workout.can_update(&user).await?;

    WorkoutBase::update(&pool, workout.id, date, user.id).await?;

    if let Some(redirect_to) = redirect_to {
        leptos_axum::redirect(&redirect_to);
    }
    Ok(())
}

#[component]
pub fn WorkoutUpdatePage() -> impl IntoView {
    let params = use_params::<WorkoutDetailParam>();
    let workout_id = move || params.with(|q| q.as_ref().map(|q| q.workout_id).unwrap_or_default());

    let action = Action::<WorkoutUpdate, _>::server();
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || extract_other_errors(action_value, &["name"]);
    let non_field_errors = move || get_non_field_errors(action_value);

    let redirect_to_url = move || {
        params.with(|p| {
            p.as_ref().map_or_else(
                |_| String::from("/"),
                |p| format!("/users/{}/workouts/{}/{}", p.username, p.date, p.workout_id),
            )
        })
    };

    let resource = Resource::new(workout_id, get_workout_detail);
    let response = move || {
        resource.and_then(|data| {
            let workout_id = data.id.to_string();
            let date = data.date.to_string();
            view! {
                <ActionForm action>
                    <input type="hidden" name="workout_id" value=workout_id/>
                    <input type="hidden" name="redirect_to" value=redirect_to_url/>
                    <TextInput action_value name="date" input_type="date" value=date/>
                    <SubmitButton loading=action_loading label="Update Workout"/>
                </ActionForm>
            }
            .into_view()
        })
    };

    view! {
        <DetailPageTemplate title="Edit Workout">
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
