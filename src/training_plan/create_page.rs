use leptos::*;
use leptos_router::*;

use crate::component::button::SubmitButton;
use crate::component::input::ValidatedInput;
use crate::error_extract::{extract_error_message, process_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user,
    setup::get_pool,
    training_plan::model::{TrainingPlan, TrainingPlanInput},
};

#[server(endpoint = "training-plan-create")]
async fn training_plan_create(
    name: String,
    duration_weeks: i32,
    redirect_to: Option<String>,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    let data = TrainingPlanInput {
        name,
        duration_weeks,
        description: None,
    };
    data.validate()?;

    let training_plan = TrainingPlan::create(&pool, user.id, &data, user.id).await?;
    if let Some(_) = redirect_to {
        leptos_axum::redirect(&format!("/training-plans/{}", training_plan.slug));
    }
    Ok(())
}

#[component]
pub fn TrainingPlanCreatePage() -> impl IntoView {
    let action = Action::<TrainingPlanCreate, _>::server();
    provide_context(action);

    let error = move || extract_error_message(&action);
    let non_field_errors = move || process_non_field_errors(error);
    view! {
        <main class="p-4 m-4 max-w-md bg-white border">
            <h1 class="text-xl font-bold">"Create Training Plan"</h1>
            <div>{non_field_errors}</div>
            <TrainingPlanCreateForm redirect_to="training-plans"/>
        </main>
    }
}

#[component]
pub fn TrainingPlanCreateForm(
    #[prop(optional, into)] redirect_to: MaybeSignal<String>,
) -> impl IntoView {
    let action = expect_context::<Action<TrainingPlanCreate, Result<(), ServerFnError>>>();

    let error = move || extract_error_message(&action);
    let non_field_errors = move || process_non_field_errors(error);
    let error = Signal::derive(error);

    view! {
        <div>{non_field_errors}</div>
        <ActionForm action class="space-y-4">
            <input type="hidden" name="redirect_to" value=redirect_to/>
            <ValidatedInput error name="name"/>
            <ValidatedInput error name="duration_weeks" value="8"/>
            <SubmitButton label="Create Training Plan" loading=action.pending()/>
        </ActionForm>
    }
}
