use leptos::*;
use leptos_router::*;

use uuid::Uuid;

use super::model::SetModel;
use crate::component::template::{
    DetailPageTemplate, ErrorComponent, LoadingComponent, UpdateDeleteButtonRow,
};
use crate::util::datetime::format_datetime;
use crate::workout::router::SetDetailParam;

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user, error::Error, exercise::model::ExerciseModel, setup::get_pool,
    workout::model::WorkoutBase,
};

#[server(endpoint = "set-detail")]
pub async fn get_set_detail(set_id: Uuid) -> Result<SetModel, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let set = SetModel::get_by_id(&pool, set_id)
        .await?
        .ok_or(Error::NotFound)?;
    let exercise = ExerciseModel::get_by_id(&pool, set.exercise_id)
        .await?
        .ok_or(Error::NotFound)?;
    let workout = WorkoutBase::get_by_id(&pool, exercise.workout_id)
        .await?
        .ok_or(Error::NotFound)?;
    workout.can_view(&user).await?;

    Ok(set)
}

#[component]
pub fn SetDetailPage() -> impl IntoView {
    let params = use_params::<SetDetailParam>();
    let set_id = move || params.with(|q| q.as_ref().map_or_else(|_| Uuid::default(), |q| q.set_id));

    let resource = Resource::new(set_id, get_set_detail);
    let response =
        move || resource.and_then(|data| view! { <SetDetailComponent data=data.clone()/> });

    view! {
        <DetailPageTemplate title="Set">
            <Transition fallback=LoadingComponent>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors/> }
                }>{response}</ErrorBoundary>
            </Transition>
        </DetailPageTemplate>
    }
}

#[component]
pub fn SetDetailComponent(data: SetModel) -> impl IntoView {
    view! {
        <header class="flex gap-2 justify-between items-start p-2 mb-2 bg-gray-200">
            <div class="px-2">
                <h2 class="font-bold">"Set: " {data.id.to_string()}</h2>
            </div>
        </header>

        <div class="mb-2">
            <table class="w-full">
                <tbody>
                    <tr>
                        <th class="p-2 w-1/2 text-left border">"Order"</th>
                        <td class="p-2 w-1/2 text-right border">{data.order}</td>
                    </tr>
                    <tr>
                        <th class="p-2 w-1/2 text-left border">"Weight"</th>
                        <td class="p-2 w-1/2 text-right border">
                            {format!("{:.2}", data.weight)} "kg"
                        </td>
                    </tr>
                    <tr>
                        <th class="p-2 w-1/2 text-left border">"Reps"</th>
                        <td class="p-2 w-1/2 text-right border">{data.reps}</td>
                    </tr>
                    <tr>
                        <th class="p-2 w-1/2 text-left border">"Rest"</th>
                        <td class="p-2 w-1/2 text-right border">{data.rest}</td>
                    </tr>
                    <tr>
                        <th class="p-2 w-1/2 text-left border">"Order"</th>
                        <td class="p-2 w-1/2 text-right border">{data.order}</td>
                    </tr>
                    <tr>
                        <th class="p-2 text-left border">"Created"</th>
                        <td class="p-2 text-right border">
                            {format_datetime(&Some(data.created_at))}
                        </td>
                    </tr>
                    <tr>
                        <th class="p-2 text-left border">"Updated"</th>
                        <td class="p-2 text-right border">{format_datetime(&data.updated_at)}</td>
                    </tr>
                </tbody>
            </table>
        </div>

        <UpdateDeleteButtonRow/>
    }
}
