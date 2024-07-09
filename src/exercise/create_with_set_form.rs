use leptos::*;
use leptos_router::*;

use rust_decimal::Decimal;
use uuid::Uuid;

use crate::component::button::Button;
use crate::component::icon::IconFilePlus;
use crate::component::input::SetInput;

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user, error::Error, exercise::model::ExerciseBase,
    set::model::SetModel, setup::get_pool, workout::model::WorkoutBase,
};

#[server(endpoint = "exercise-set-create")]
pub async fn exercise_set_create(
    workout_id: Uuid,
    movement_id: Uuid,
    weight: Decimal,
    reps: i32,
    rest: i32,
    set_count: i32,
    redirect_to: Option<String>,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    let workout = WorkoutBase::get_by_id(&pool, workout_id)
        .await?
        .ok_or(Error::NotFound)?;
    workout.can_update(&user).await?;
    let exercise = ExerciseBase::create(&pool, workout.id, movement_id, user.id).await?;
    SetModel::bulk_create(&pool, exercise.id, weight, reps, rest, set_count, user.id).await?;
    if let Some(redirect_to) = redirect_to {
        leptos_axum::redirect(&redirect_to);
    }
    Ok(())
}

#[component]
pub fn ExerciseSetCreateForm(
    redirect_to: String,
    workout_id: String,
    movement_id: String,
    weight: i32,
    sets: i32,
    reps: i32,
) -> impl IntoView {
    let action = expect_context::<Action<ExerciseSetCreate, Result<(), ServerFnError>>>();
    view! {
        <ActionForm action class="contents">
            <div class="p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
                <SetInput name="weight" label="kg" value=weight/>
            </div>
            <div class="p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
                <SetInput name="set_count" label="sets" value=sets/>
            </div>
            <div class="p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
                <SetInput name="reps" label="reps" value=reps/>
            </div>
            <div class="flex justify-end items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
                <input type="hidden" name="redirect_to" value=redirect_to/>
                <input type="hidden" name="workout_id" value=workout_id/>
                <input type="hidden" name="movement_id" value=movement_id/>
                <input type="hidden" name="rest" value="0"/>
                <Button loading=action.pending() label="Add">
                    <IconFilePlus/>
                </Button>
            </div>
        </ActionForm>
    }
}
