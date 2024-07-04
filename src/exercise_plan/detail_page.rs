use leptos::*;
use leptos_router::*;

use uuid::Uuid;

use crate::component::template::{
    DetailPageTemplate, ErrorComponent, LoadingComponent, UpdateDeleteButtonRow,
};
use crate::exercise_plan::model::ExercisePlanQuery;
use crate::training_plan::router::ExercisePlanDetailParam;
use crate::util::datetime::format_datetime;

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, setup::get_pool};

#[server(endpoint = "exercise-plan-detail")]
pub async fn get_exercise_plan_detail(id: Uuid) -> Result<ExercisePlanQuery, ServerFnError> {
    let _user = get_request_user()?;
    let pool = get_pool()?;

    let query = ExercisePlanQuery::get(&pool, id).await?;
    Ok(query)
}

#[component]
pub fn ExercisePlanDetailPage() -> impl IntoView {
    let params = use_params::<ExercisePlanDetailParam>();
    let id = move || params.with(|p| p.as_ref().map(|p| p.exercise_id).unwrap_or_default());

    let resource = Resource::new(id, get_exercise_plan_detail);
    let response = move || {
        resource.and_then(|data| {
            view! { <ExercisePlanDetailComponent data/> }
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

#[component]
pub fn ExercisePlanDetailComponent<'a>(data: &'a ExercisePlanQuery) -> impl IntoView {
    let created_at = format_datetime(&Some(data.created_at));
    let updated_at = format_datetime(&data.updated_at);
    view! {
        <table class="mb-4 w-full border-collapse">
            <tbody>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Exercise"</th>
                    <td class="p-2 w-1/2 text-right border">{&data.movement_name}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Workout Plan"</th>
                    <td class="p-2 w-1/2 text-right border">{&data.workout_plan_name}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Sequence"</th>
                    <td class="p-2 w-1/2 text-right border">{data.sequence}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Sets"</th>
                    <td class="p-2 w-1/2 text-right border">{data.sets}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Reps"</th>
                    <td class="p-2 w-1/2 text-right border">{data.reps}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Created"</th>
                    <td class="p-2 w-1/2 text-right border">{created_at}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Updated"</th>
                    <td class="p-2 w-1/2 text-right border">{updated_at}</td>
                </tr>
            </tbody>
        </table>
        <UpdateDeleteButtonRow/>
    }
}
