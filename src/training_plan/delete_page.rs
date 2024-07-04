use leptos::*;
use leptos_router::*;

use uuid::Uuid;

use super::detail_page::get_training_plan_detail;
use crate::component::button::{Button, ButtonVariant};
use crate::component::template::{DeletePageWrapper, ErrorComponent, LoadingComponent};
use crate::util::param::extract_param;

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, setup::get_pool, training_plan::model::TrainingPlan};

#[server(endpoint = "training-plan-delete")]
pub async fn training_plan_delete(
    id: Uuid,
    redirect_to: Option<String>,
) -> Result<(), ServerFnError> {
    let _user = get_request_user()?;
    let pool = get_pool()?;
    let training_plan = TrainingPlan::get_object_or_404(&pool, id).await?;
    TrainingPlan::delete(&pool, training_plan.id).await?;
    if let Some(redirect_to) = redirect_to {
        leptos_axum::redirect(&redirect_to);
    }
    Ok(())
}

#[component]
pub fn TrainingPlanDeletePage() -> impl IntoView {
    let action = Action::<TrainingPlanDelete, _>::server();
    provide_context(action);

    let params = use_params_map();
    let slug = move || extract_param(&params, "training_slug");

    let resource = Resource::new(slug, get_training_plan_detail);
    let response = move || {
        resource.and_then(|data| {
            let id = data.id.to_string();
            view! { <TrainingPlanDeleteForm id redirect_to="training-plans"/> }
        })
    };
    view! {
        <DeletePageWrapper title="Training Plan">
            <Transition fallback=LoadingComponent>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors/> }
                }>{response}</ErrorBoundary>
            </Transition>
        </DeletePageWrapper>
    }
}

#[component]
pub fn TrainingPlanDeleteForm(
    #[prop(optional, into)] id: MaybeSignal<String>,
    #[prop(optional, into)] redirect_to: MaybeSignal<String>,
) -> impl IntoView {
    let action = expect_context::<Action<TrainingPlanDelete, _>>();
    let loading = action.pending();
    view! {
        <ActionForm action class="contents">
            <input type="hidden" name="id" value=id/>
            <input type="hidden" name="redirect_to" value=redirect_to/>
            <Button label="Delete Training Plan" variant=ButtonVariant::Danger loading/>
        </ActionForm>
    }
}
