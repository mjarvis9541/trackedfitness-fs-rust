use leptos::*;
use leptos_router::*;

use crate::component::button::SubmitButton;
use crate::component::input::ValidatedInput;
use crate::component::template::DetailPageTemplate;
use crate::component::toast::ToastSuccess;
use crate::error_extract::{extract_error_message, process_non_field_errors};
use crate::training_plan::select::TrainingPlanSelect;
use crate::workout_plan::model::WorkoutPlan;

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user, setup::get_pool, workout_plan::model::WorkoutPlanInput,
};

#[server(endpoint = "workout-plan-create")]
async fn workout_plan_create(name: String) -> Result<WorkoutPlan, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let data = WorkoutPlanInput {
        user_id: user.id,
        name,
    };
    data.validate()?;
    let query = WorkoutPlan::create(&pool, &data, user.id).await?;
    Ok(query)
}

#[component]
pub fn WorkoutPlanCreatePage() -> impl IntoView {
    let action = Action::<WorkoutPlanCreate, _>::server();
    provide_context(action);

    view! {
        <DetailPageTemplate title="New Workout Plan">
            <WorkoutPlanCreateForm/>
        </DetailPageTemplate>
    }
}

#[component]
pub fn WorkoutPlanCreateForm() -> impl IntoView {
    let action = expect_context::<Action<WorkoutPlanCreate, Result<WorkoutPlan, ServerFnError>>>();
    let result_ok = move || {
        action
            .value()
            .with(|x| x.as_ref().is_some_and(|y| y.is_ok()))
    };
    view! {
        <ToastSuccess update_show=Signal::derive(result_ok)/>
        <div>
            <header class="mb-4">
                <h1 class="text-xl font-bold">"Create Workout Plan"</h1>
            </header>
            <ActionForm action class="space-y-4">
                <ValidatedInput name="name"/>
                <TrainingPlanSelect/>
                <ValidatedInput name="sequence" value="1"/>
                <ValidatedInput name="weekday" value="1"/>
                <SubmitButton loading=action.pending() label="Create Workout Plan"/>
            </ActionForm>
        </div>
    }
}

#[component]
pub fn WorkoutPlanAddForm(
    #[prop(optional, into)] training_plan_id: MaybeSignal<String>,
    #[prop(optional, into)] next_workout_seq: MaybeSignal<String>,
    #[prop(optional, into)] class: MaybeSignal<String>,
) -> impl IntoView {
    let action = expect_context::<Action<WorkoutPlanCreate, _>>();
    let error = move || extract_error_message(&action);
    let non_field_errors = move || process_non_field_errors(error);
    let error = Signal::derive(error);

    view! {
        <div>{non_field_errors}</div>
        <ActionForm action class=class>
            <input type="hidden" name="training_plan_id" value=training_plan_id/>
            <ValidatedInput error name="name"/>
            <ValidatedInput error name="sequence" value=next_workout_seq/>
            <ValidatedInput error name="weekday" value="0"/>
            <SubmitButton loading=action.pending() label="Add Workout"/>
        </ActionForm>
    }
}
