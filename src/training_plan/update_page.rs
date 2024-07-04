use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use uuid::Uuid;

use super::detail_page::get_training_plan_detail;
use crate::component::button::SubmitButton;
use crate::component::input::ValidatedInput;
use crate::component::template::{ErrorComponent, LoadingComponent};
use crate::training_plan::model::TrainingPlanQuery;
use crate::util::param::extract_param;

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user,
    setup::get_pool,
    training_plan::model::{TrainingPlan, TrainingPlanInput},
};

#[server(endpoint = "training-plan-update")]
pub async fn training_plan_update(
    id: Uuid,
    name: String,
    duration_weeks: i32,
    redirect_to: Option<String>,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    let _training_plan = TrainingPlan::get_object_or_404(&pool, id).await?;
    let data = TrainingPlanInput {
        name,
        duration_weeks,
        description: None,
    };
    data.validate()?;
    let training_plan = TrainingPlan::update(&pool, user.id, &data, user.id).await?;
    if let Some(_) = redirect_to {
        leptos_axum::redirect(&format!("/training-plans/{}", training_plan.slug));
    }
    Ok(())
}

#[component]
pub fn TrainingPlanUpdatePage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || extract_param(&params, "training_slug");

    let action = Action::<TrainingPlanUpdate, _>::server();
    let pending = action.pending();

    let resource = Resource::new(slug, get_training_plan_detail);

    let response = move || {
        resource.and_then(|data| {
            let id = data.id.to_string();
            let name = data.name.clone();
            let duration = data.duration_weeks.to_string();
            view! {
                <ActionForm action class="space-y-4">
                    <input type="hidden" name="id" value=id/>
                    <input type="hidden" name="redirect_to" value="/training-plans"/>
                    <ValidatedInput name="name" value=name/>
                    <ValidatedInput name="duration_weeks" value=duration/>
                    <SubmitButton loading=pending/>
                </ActionForm>
            }
        })
    };

    view! {
        <Title text="Update Training Plan"/>
        <main class="p-4">
            <div class="p-4 max-w-md bg-white border">
                <h1 class="mb-4 text-xl font-bold">"Update Training Plan"</h1>
                <section>
                    <Transition fallback=LoadingComponent>
                        <ErrorBoundary fallback=|errors| {
                            view! { <ErrorComponent errors/> }
                        }>{response}</ErrorBoundary>
                    </Transition>
                </section>
            </div>
        </main>
    }
}

#[component]
pub fn TrainingPlanUpdateForm(data: TrainingPlanQuery) -> impl IntoView {
    let action = Action::<TrainingPlanUpdate, _>::server();
    let id = data.id.to_string();
    let name = data.name.clone();
    let duration = data.duration_weeks.to_string();
    view! {
        <ActionForm action class="space-y-4">
            <input type="hidden" name="id" value=id/>
            <input type="hidden" name="redirect_to" value="/training-plans"/>
            <ValidatedInput name="name" value=name/>
            <ValidatedInput name="duration_weeks" value=duration/>
            <SubmitButton loading=action.pending()/>
        </ActionForm>
    }
}
