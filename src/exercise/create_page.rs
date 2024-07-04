use leptos::*;
use leptos_router::*;

use uuid::Uuid;

use crate::component::button::Button;
use crate::component::icon::IconFilePlus;

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user, error::Error, exercise::model::ExerciseModel, setup::get_pool,
    workout::model::WorkoutBase,
};

#[server(endpoint = "exercise-create")]
pub async fn exercise_create(
    workout_id: Uuid,
    movement_id: Uuid,
    redirect_to: Option<String>,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let workout = WorkoutBase::get_by_id(&pool, workout_id)
        .await?
        .ok_or(Error::NotFound)?;
    workout.can_update(&user).await?;

    ExerciseModel::create(&pool, workout.id, movement_id, user.id).await?;
    if let Some(redirect_to) = redirect_to {
        leptos_axum::redirect(&redirect_to);
    }
    Ok(())
}

#[component]
pub fn ExerciseCreateForm(
    redirect_to: String,
    workout_id: String,
    movement_id: String,
) -> impl IntoView {
    let action = expect_context::<Action<ExerciseCreate, Result<(), ServerFnError>>>();
    view! {
        <ActionForm action class="contents">
            <div class="flex justify-end items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
                <input type="hidden" name="redirect_to" value=redirect_to/>
                <input type="hidden" name="workout_id" value=workout_id/>
                <input type="hidden" name="movement_id" value=movement_id/>
                <Button loading=action.pending() label="Add Without Sets">
                    <IconFilePlus/>
                </Button>
            </div>
        </ActionForm>
    }
}
