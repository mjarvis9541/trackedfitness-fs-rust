use leptos::*;
use leptos_router::*;

use uuid::Uuid;

use crate::component::button::{Button, ButtonVariant};
use crate::component::icon::IconTrash;
use crate::component::template::{DetailPageTemplate, ErrorComponent, LoadingComponent};
use crate::exercise_plan::detail_page::get_exercise_plan_detail;
use crate::training_plan::router::ExercisePlanDetailParam;

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user, exercise_plan::model::ExercisePlan, setup::get_pool,
    workout_plan::model::WorkoutPlan,
};

#[server(endpoint = "exercise-plan-delete")]
pub async fn exercise_plan_delete(
    id: Uuid,
    redirect_to: Option<String>,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let exercise_plan = ExercisePlan::get_object_or_404(&pool, id).await?;

    let workout_plan = WorkoutPlan::get_object_or_404(&pool, exercise_plan.workout_plan_id).await?;
    workout_plan.can_update(&user).await?;

    ExercisePlan::delete(&pool, exercise_plan.id).await?;

    if let Some(redirect_to) = redirect_to {
        leptos_axum::redirect(&redirect_to);
    }
    Ok(())
}

#[component]
pub fn ExercisePlanDeletePage() -> impl IntoView {
    let action = Action::<ExercisePlanDelete, _>::server();
    provide_context(action);

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
    let redirect_href = Signal::derive(redirect_href);

    let response = move || {
        resource.and_then(|data| {
            let id = data.id.to_string();
            view! { <ExercisePlanDeleteForm id redirect_to=redirect_href/> }
        })
    };

    view! {
        <DetailPageTemplate title="Delete Exercise Plan">
            <Transition fallback=LoadingComponent>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors/> }
                }>{response}</ErrorBoundary>
            </Transition>
        </DetailPageTemplate>
    }
}

#[component]
pub fn ExercisePlanDeleteForm(
    #[prop(optional, into)] id: MaybeSignal<String>,
    #[prop(optional, into)] redirect_to: MaybeSignal<String>,
) -> impl IntoView {
    let action = expect_context::<Action<ExercisePlanDelete, _>>();
    view! {
        <p class="mb-4">"Are you sure you wish to delete this exercise plan?"</p>
        <p class="mb-4">"Ths action cannot be undone."</p>
        <ActionForm action>
            <input type="hidden" name="id" value=id/>
            <input type="hidden" name="redirect_to" value=redirect_to/>
            <Button variant=ButtonVariant::Danger loading=action.pending()>
                <IconTrash/>
            </Button>
        </ActionForm>
    }
}
