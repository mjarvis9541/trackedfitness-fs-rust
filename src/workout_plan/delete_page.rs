use leptos::*;
use leptos_router::*;

use uuid::Uuid;

use crate::component::button::{Button, ButtonVariant};
use crate::component::icon::IconTrash;
use crate::component::template::{DeletePageWrapper, ErrorComponent, LoadingComponent};
use crate::training_plan::router::WorkoutPlanDetailParam;
use crate::workout_plan::detail_page::get_workout_plan_detail;

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user, error::Error, setup::get_pool,
    workout_plan::model::WorkoutPlan,
};

#[server(endpoint = "workout-plan-delete")]
pub async fn workout_plan_delete(
    id: Uuid,
    redirect_to: Option<String>,
) -> Result<(), ServerFnError> {
    let _user = get_request_user()?;
    let pool = get_pool()?;

    let workout_plan = WorkoutPlan::get_object_or_404(&pool, id).await?;
    WorkoutPlan::delete(&pool, workout_plan.id).await?;
    if let Some(redirect_to) = redirect_to {
        leptos_axum::redirect(&redirect_to);
    }
    Ok(())
}

#[component]
pub fn WorkoutPlanDeletePage() -> impl IntoView {
    let action = Action::<WorkoutPlanDelete, _>::server();
    provide_context(action);

    let params = use_params::<WorkoutPlanDetailParam>();
    let slug = move || {
        params.with(|p| {
            p.as_ref()
                .map(|p| p.workout_slug.clone())
                .unwrap_or_default()
        })
    };
    let resource = Resource::new(slug, get_workout_plan_detail);
    let response = move || {
        resource.and_then(|data| {
            let id = data.id.to_string();
            view! { <WorkoutPlanDeleteForm id redirect_to="training-plans"/> }
        })
    };
    view! {
        <DeletePageWrapper title="Workout Plan">
            <Transition fallback=LoadingComponent>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors/> }
                }>{response}</ErrorBoundary>
            </Transition>
        </DeletePageWrapper>
    }
}

#[component]
pub fn WorkoutPlanDeleteForm(
    #[prop(optional, into)] id: MaybeSignal<String>,
    #[prop(optional, into)] redirect_to: MaybeSignal<String>,
) -> impl IntoView {
    let action = expect_context::<Action<WorkoutPlanDelete, _>>();

    view! {
        <ActionForm action>
            <input type="hidden" name="id" value=id/>
            <input type="hidden" name="redirect_to" value=redirect_to/>
            <Button variant=ButtonVariant::Danger loading=action.pending()>
                <IconTrash/>
            </Button>
        </ActionForm>
    }
}
