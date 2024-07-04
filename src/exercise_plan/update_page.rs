use leptos::*;
use leptos_router::*;

use rust_decimal::Decimal;
use uuid::Uuid;

use crate::component::button::{Button, ButtonVariant};
use crate::component::input::ValidatedInput;
use crate::component::template::{DetailPageTemplate, ErrorComponent, LoadingComponent};
use crate::exercise_plan::detail_page::get_exercise_plan_detail;
use crate::movement::select::{get_movement_select, MovementSelect};
use crate::training_plan::router::ExercisePlanDetailParam;

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user,
    exercise_plan::model::{ExercisePlan, ExercisePlanInput},
    setup::get_pool,
    workout_plan::model::WorkoutPlan,
};

#[server(endpoint = "exercise-plan-update")]
pub async fn exercise_plan_update(
    id: Uuid,
    workout_plan_id: Uuid,
    movement_id: Uuid,
    sequence: i32,
    weight: Decimal,
    sets: i32,
    reps: i32,
    rest: i32,
    redirect_to: Option<String>,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let _workout_plan = WorkoutPlan::get_object_or_404(&pool, workout_plan_id).await?;

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
    ExercisePlan::update(&pool, id, &data, user.id).await?;
    if let Some(redirect_to) = redirect_to {
        leptos_axum::redirect(&redirect_to);
    }
    Ok(())
}

#[component]
pub fn ExercisePlanUpdatePage() -> impl IntoView {
    let action = Action::<ExercisePlanUpdate, _>::server();

    let movement_resource = Resource::once(get_movement_select);
    provide_context(movement_resource);

    let params = use_params::<ExercisePlanDetailParam>();
    let id = move || params.with(|p| p.as_ref().map(|p| p.exercise_id).unwrap_or_default());

    let resource = Resource::new(id, get_exercise_plan_detail);

    let redirect_href = move || {
        params.with(|p| {
            p.as_ref()
                .map(|p| {
                    if let Some(training_slug) = &p.training_slug {
                        format!("/training-plans/{}", training_slug)
                    } else {
                        format!("/training-plans/workout-plans/{}", p.workout_slug)
                    }
                })
                .unwrap_or_default()
        })
    };

    let response = move || {
        resource.and_then(|data| {
            let id = data.id.to_string();
            let workout_id = data.workout_plan_id.to_string();
            let sequence = data.sequence.to_string();
            let weight = data.weight.to_string();
            let sets = data.sets.to_string();
            let reps = data.reps.to_string();
            let rest = data.rest.to_string();
            let movement_id = data.movement_id;

            view! {
                <ActionForm action class="space-y-4">
                    <input type="hidden" name="id" value=id/>
                    <input type="hidden" name="workout_plan_id" value=workout_id/>
                    <input type="hidden" name="redirect_to" value=redirect_href/>

                    <MovementSelect name="movement_id" selected=movement_id/>
                    <ValidatedInput name="sequence" value=sequence/>
                    <ValidatedInput name="weight" value=weight/>
                    <ValidatedInput name="sets" value=sets/>
                    <ValidatedInput name="reps" value=reps/>
                    <ValidatedInput name="rest" value=rest/>
                    <Button
                        variant=ButtonVariant::Primary
                        label="Update Exercise"
                        loading=action.pending()
                    />
                </ActionForm>
            }
        })
    };

    view! {
        <DetailPageTemplate title="Exercise Plan">
            <Transition fallback=LoadingComponent>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors/> }
                }>{response}</ErrorBoundary>
            </Transition>
        </DetailPageTemplate>
    }
}
