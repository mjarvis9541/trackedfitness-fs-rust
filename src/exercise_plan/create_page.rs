use leptos::*;
use leptos_router::*;

use rust_decimal::Decimal;
use uuid::Uuid;

use crate::component::button::SubmitButton;
use crate::component::input::ValidatedInput;
use crate::error_extract::extract_error_message;
use crate::movement::select::MovementSelect;

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user,
    exercise_plan::model::{ExercisePlan, ExercisePlanInput},
    setup::get_pool,
    workout_plan::model::WorkoutPlan,
};

#[server(endpoint = "exercise-plan-create")]
async fn exercise_plan_create(
    workout_plan_id: Uuid,
    movement_id: Uuid,
    sequence: i32,
    weight: Decimal,
    sets: i32,
    reps: i32,
    rest: i32,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let workout_plan = WorkoutPlan::get_object_or_404(&pool, workout_plan_id).await?;
    workout_plan.can_update(&user).await?;

    let data = ExercisePlanInput {
        workout_plan_id,
        movement_id,
        sequence,
        weight,
        sets,
        reps,
        rest,
    };
    data.validate()?;

    ExercisePlan::create(&pool, &data, user.id).await?;

    Ok(())
}

#[component]
pub fn ExercisePlanCreateForm(
    #[prop(optional, into)] workout_plan_id: Signal<String>,
    #[prop(optional, into)] next_exercise_seq: Signal<String>,
) -> impl IntoView {
    let action = expect_context::<Action<ExercisePlanCreate, _>>();
    let next_seq = next_exercise_seq.get();

    view! {
        <div>
            <header class="mb-4">
                <h1 class="text-xl font-bold">"Add Exercise"</h1>
            </header>
            <ActionForm action>
                <input type="hidden" name="workout_plan_id" value=workout_plan_id/>
                <MovementSelect name="movement_id"/>
                <ValidatedInput name="sequence" value=next_seq/>
                <ValidatedInput name="weight" value="60"/>
                <ValidatedInput name="sets" value="3"/>
                <ValidatedInput name="reps" value="10"/>
                <ValidatedInput name="rest" value="0"/>
                <button class="py-1.5 px-3 bg-gray-100 hover:bg-gray-200">"Add Exercise"</button>
            </ActionForm>
        </div>
    }
}

#[component]
pub fn AddExercisePlanForm(
    #[prop(optional, into)] workout_plan_id: MaybeSignal<String>,
    #[prop(optional, into)] next_exercise_seq: MaybeSignal<String>,
) -> impl IntoView {
    let action = expect_context::<Action<ExercisePlanCreate, _>>();
    let error = move || extract_error_message(&action);

    view! {
        <h3 class="mb-2 text-base font-bold">"Add Exercise"</h3>
        <ActionForm action>
            <input type="hidden" name="workout_plan_id" value=workout_plan_id/>
            <input type="hidden" name="sequence" value=next_exercise_seq/>
            <div class="flex flex-wrap gap-2 mb-2">
                <div class="flex-1">
                    <MovementSelect name="movement_id"/>
                </div>
                <div class="flex-1">
                    <ValidatedInput name="weight" value="60" error=Signal::derive(error)/>
                </div>
            </div>
            <div class="flex flex-wrap gap-2 mb-2">
                <div class="flex-1">
                    <ValidatedInput name="sets" value="3" error=Signal::derive(error)/>
                </div>
                <div class="flex-1">
                    <ValidatedInput name="reps" value="10" error=Signal::derive(error)/>
                </div>
                <div class="flex-1">
                    <ValidatedInput name="rest" value="0" error=Signal::derive(error)/>
                </div>
            </div>
            <SubmitButton label="Add Exercise" loading=action.pending()/>
        </ActionForm>
    }
}
