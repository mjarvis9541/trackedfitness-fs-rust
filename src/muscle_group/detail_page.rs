use leptos::server_fn::codec::GetUrl;
use leptos::*;
use leptos_router::*;

use crate::auth::context::RequestUserContext;
use crate::component::template::{
    DetailPageTemplate, ErrorComponent, LoadingComponent, UpdateDeleteButtonRow,
};
use crate::util::datetime::format_datetime;
use crate::util::param::get_slug;

use super::model::MuscleGroup;

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, error::Error, setup::get_pool};

#[server(endpoint = "get-muscle-group", input = GetUrl)]
pub async fn get_muscle_group(slug: String) -> Result<MuscleGroup, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    let object = MuscleGroup::get_by_slug(&pool, &slug)
        .await?
        .ok_or(Error::NotFound)?;
    object.can_view(&user)?;
    Ok(object)
}

#[component]
pub fn MuscleGroupDetailPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || get_slug(&params);

    let resource = Resource::new(slug, get_muscle_group);
    let response =
        move || resource.and_then(|data| view! { <MuscleGroupDetail data=data.clone()/> });

    view! {
        <DetailPageTemplate title="Muscle Group">
            <Transition fallback=LoadingComponent>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors/> }
                }>{response}</ErrorBoundary>
            </Transition>
        </DetailPageTemplate>
    }
}

#[component]
pub fn MuscleGroupDetail(data: MuscleGroup) -> impl IntoView {
    let created_at = format_datetime(&Some(data.created_at));
    let updated_at = format_datetime(&data.updated_at);

    let updated_by = data.updated_by.map_or_else(
        || "-".into_view(),
        |user| {
            view! {
                <a class="text-blue-500 hover:underline" href=format!("/users/{}", user)>
                    {user}
                </a>
            }
            .into_view()
        },
    );

    let user = expect_context::<RequestUserContext>();
    let can_edit = move || user.is_superuser_or_object_owner(data.created_by_id);

    view! {
        <h1 class="mb-4 text-xl font-bold">{data.name}</h1>

        <table class="mb-4 w-full border-collapse">
            <tbody>
                <tr>
                    <th class="py-2 pl-2 w-1/2 text-left border">"Slug"</th>
                    <td class="py-2 pr-2 w-1/2 text-right border">{data.slug}</td>
                </tr>
                <tr>
                    <th class="py-2 pl-2 w-1/2 text-left border">"Created by"</th>
                    <td class="py-2 pr-2 w-1/2 text-right border">
                        <a
                            class="text-blue-500 hover:underline"
                            href=format!("/users/{}", data.created_by)
                        >
                            {data.created_by}
                        </a>
                    </td>
                </tr>
                <tr>
                    <th class="py-2 pl-2 w-1/2 text-left border">"Updated by"</th>
                    <td class="py-2 pr-2 w-1/2 text-right border">{updated_by}</td>
                </tr>
                <tr>
                    <th class="py-2 pl-2 w-1/2 text-left border">"Created"</th>
                    <td class="py-2 pr-2 w-1/2 text-right border">{created_at}</td>
                </tr>
                <tr>
                    <th class="py-2 pl-2 w-1/2 text-left border">"Updated"</th>
                    <td class="py-2 pr-2 w-1/2 text-right border">{updated_at}</td>
                </tr>
            </tbody>
        </table>

        <Show when=can_edit>
            <UpdateDeleteButtonRow/>
        </Show>
    }
}
