use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use uuid::Uuid;

use crate::component::button::{Button, ButtonVariant};
use crate::component::input::ValidatedInput;
use crate::component::template::{ErrorComponent, LoadingComponent};
use crate::training_plan::router::WorkoutPlanDetailParam;
use crate::workout_plan::detail_page::get_workout_plan_detail;

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user,
    setup::get_pool,
    workout_plan::model::{WorkoutPlan, WorkoutPlanInput},
};

#[server(endpoint = "workout-plan-update")]
pub async fn workout_plan_update(
    id: Uuid,
    name: String,
    redirect_to: Option<String>,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    let workout_plan = WorkoutPlan::get_object_or_404(&pool, id).await?;
    let data = WorkoutPlanInput {
        user_id: user.id,
        name,
    };
    data.validate()?;
    let workout_plan = WorkoutPlan::update(&pool, workout_plan.id, &data, user.id).await?;
    if let Some(_redirect_to) = redirect_to {
        leptos_axum::redirect(&format!(
            "/training-plans/workout-plans/{}",
            workout_plan.id
        ));
    }
    Ok(())
}

#[component]
pub fn WorkoutPlanUpdatePage() -> impl IntoView {
    let action = Action::<WorkoutPlanUpdate, _>::server();

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
            let name = data.name.clone();
            view! {
                <ActionForm action class="space-y-4">
                    <input type="hidden" name="id" value=id/>
                    <input type="hidden" name="redirect_to" value="redirect"/>
                    <ValidatedInput name="name" value=name/>
                    <Button
                        variant=ButtonVariant::Primary
                        label="Update Workout Plan"
                        loading=action.pending()
                    />
                </ActionForm>
            }
        })
    };

    view! {
        <Title text="Edit Workout Plan"/>
        <main class="p-4">
            <div class="p-4 max-w-md bg-white border">
                <h1 class="mb-4 text-xl font-bold">"Update Workout Plan"</h1>
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
