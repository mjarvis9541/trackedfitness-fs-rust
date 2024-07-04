use leptos::*;
use leptos_router::*;

use rust_decimal::Decimal;
use uuid::Uuid;

use crate::component::button::Button;
use crate::component::icon::IconFilePlus;

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user, error::Error, exercise::model::ExerciseModel,
    set::model::SetModel, setup::get_pool, workout::model::WorkoutBase,
};

#[server(endpoint = "set-create")]
pub async fn set_create(
    exercise_id: Uuid,
    order: i32,
    weight: Option<Decimal>,
    reps: Option<i32>,
    rest: Option<i32>,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let exercise = ExerciseModel::get_by_id(&pool, exercise_id)
        .await?
        .ok_or(Error::NotFound)?;
    let workout = WorkoutBase::get_by_id(&pool, exercise.workout_id)
        .await?
        .ok_or(Error::NotFound)?;
    workout.can_update(&user).await?;

    let weight = weight.unwrap_or_default();
    let reps = reps.unwrap_or_default();
    let rest = rest.unwrap_or_default();

    SetModel::validate(order, weight, reps, rest)?;
    SetModel::create(&pool, exercise_id, order, weight, reps, rest, user.id).await?;
    Ok(())
}

#[component]
pub fn SetCreateForm(exercise_id: String, order: i64, weight: String, reps: i32) -> impl IntoView {
    let action = expect_context::<Action<SetCreate, Result<(), ServerFnError>>>();
    view! {
        <ActionForm action class="contents">
            <input type="hidden" name="exercise_id" value=exercise_id/>
            <input type="hidden" name="order" value=order/>
            <input type="hidden" name="weight" value=weight/>
            <input type="hidden" name="reps" value=reps/>
            <Button label="Add Set">
                <IconFilePlus/>
            </Button>
        </ActionForm>
    }
}
