use leptos::*;
use leptos_router::*;

use uuid::Uuid;

use crate::component::template::{
    DetailPageTemplate, ErrorComponent, LoadingComponent, UpdateDeleteButtonRow,
};
use crate::util::datetime::format_datetime;

use super::model::WorkoutBase;
use super::router::WorkoutDetailParam;

#[cfg(feature = "ssr")]
use crate::{auth::model::User, auth::service::get_request_user, error::Error, setup::get_pool};

#[server(endpoint = "workout-detail")]
pub async fn get_workout_detail(workout_id: Uuid) -> Result<WorkoutBase, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let workout = WorkoutBase::get_by_id(&pool, workout_id)
        .await?
        .ok_or(Error::NotFound)?;

    User::check_view_permission_by_user_id(&pool, &user, workout.user_id).await?;

    Ok(workout)
}

#[component]
pub fn WorkoutDetailPage() -> impl IntoView {
    let params = use_params::<WorkoutDetailParam>();
    let workout_id = move || params.with(|q| q.as_ref().map(|q| q.workout_id).unwrap_or_default());

    let resource = Resource::new(workout_id, get_workout_detail);
    let response = move || resource.and_then(|data| view! { <WorkoutDetail data=data.clone()/> });
    view! {
        <DetailPageTemplate title="Workout">
            <Transition fallback=LoadingComponent>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors/> }
                }>{response}</ErrorBoundary>
            </Transition>
        </DetailPageTemplate>
    }
}

#[component]
pub fn WorkoutDetail(data: WorkoutBase) -> impl IntoView {
    let created_at = format_datetime(&Some(data.created_at));
    let updated_at = format_datetime(&data.updated_at);

    view! {
        <header class="mb-4">
            <h1 class="text-xl font-bold">{data.date.to_string()}</h1>
            <p class="text-gray-500">{data.id.to_string()}</p>
        </header>
        <table class="mb-4 w-full border-collapse">
            <tbody>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Created by"</th>
                    <td class="p-2 w-1/2 text-right border">{data.created_by_id.to_string()}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Updated by"</th>
                    <td class="p-2 w-1/2 text-right border">
                        {data.updated_by_id.map_or_else(|| "-".to_string(), |d| d.to_string())}
                    </td>
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
