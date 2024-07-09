use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use uuid::Uuid;

use crate::component::button::SubmitButton;
use crate::component::select::FieldSelect;
use crate::component::template::{ErrorComponent, LoadingComponent};
use crate::follower::model::Follower;
use crate::util::datetime::format_datetime;
use crate::util::param::UuidParam;

use super::user_select::{get_user_select, UserSelect, UserSelectResource};

#[cfg(feature = "ssr")]
use crate::{auth::service::extract_superuser_from_request, error::Error, setup::get_pool};

#[server(endpoint = "admin-follower-update")]
pub async fn admin_follower_update(
    id: Uuid,
    user_id: Uuid,
    follower_id: Uuid,
    status: i32,
) -> Result<(), ServerFnError> {
    extract_superuser_from_request()?;
    let pool = get_pool()?;
    Follower::update(&pool, id, user_id, follower_id, status).await?;
    Ok(())
}

#[server(endpoint = "admin-follower-detail")]
pub async fn get_admin_follower_detail(id: Uuid) -> Result<Follower, ServerFnError> {
    extract_superuser_from_request()?;
    let pool = get_pool()?;
    let query = Follower::get_by_id(&pool, id)
        .await?
        .ok_or(Error::NotFound)?;
    Ok(query)
}

#[component]
pub fn AdminFollowerDetailPage() -> impl IntoView {
    let resource: UserSelectResource = Resource::once(get_user_select);
    provide_context(resource);

    let action = Action::<AdminFollowerUpdate, _>::server();
    let params = use_params::<UuidParam>();
    let id = move || params.with(|q| q.as_ref().map(|q| q.id).unwrap_or_default());

    let resource = Resource::new(
        move || (id(), action.version().get()),
        |(id, _)| get_admin_follower_detail(id),
    );

    let response = move || {
        resource.and_then(|data| view! { <AdminFollowerDetailComponent data=data.clone()/> })
    };
    let form_response = move || {
        resource.and_then(|data| view! { <AdminFollowerUpdateForm data=data.clone() action/> })
    };

    view! {
        <Title text="Admin - Follower Detail"/>
        <main class="grid grid-cols-4 gap-4 p-4 md:grid-cols-8 lg:grid-cols-12">

            <section class="col-span-4">
                <div class="p-4 bg-white border">
                    <h1 class="mb-4 text-xl font-bold">"Admin - Follower Detail"</h1>
                    <Transition fallback=LoadingComponent>
                        <ErrorBoundary fallback=|errors| {
                            view! { <ErrorComponent errors/> }
                        }>{response}</ErrorBoundary>
                    </Transition>
                </div>
            </section>

            <section class="col-span-4">
                <div class="p-4 mb-4 bg-white">
                    <h2 class="mb-4 text-xl font-bold">"Update User Follower"</h2>

                    <Transition fallback=LoadingComponent>
                        <ErrorBoundary fallback=|errors| {
                            view! { <ErrorComponent errors/> }
                        }>{form_response}</ErrorBoundary>
                    </Transition>
                </div>
            </section>

        </main>
    }
}

#[component]
pub fn AdminFollowerDetailComponent(data: Follower) -> impl IntoView {
    let created_at = format_datetime(&Some(data.created_at));
    let updated_at = format_datetime(&data.updated_at);

    view! {
        <table class="w-full border-collapse">
            <tbody>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Username"</th>
                    <td class="p-2 w-1/2 text-right border">{data.username}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Follower"</th>
                    <td class="p-2 w-1/2 text-right border">{data.follower}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Status"</th>
                    <td class="p-2 w-1/2 text-right border">{data.status.to_string()}</td>
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
    }
}

#[component]
pub fn AdminFollowerUpdateForm(
    data: Follower,
    action: Action<AdminFollowerUpdate, Result<(), ServerFnError>>,
) -> impl IntoView {
    view! {
        <ActionForm action>
            <input type="hidden" name="id" value=data.id.to_string()/>
            <UserSelect name="user_id" label="user" selected=data.user_id/>
            <UserSelect name="follower_id" label="follower" selected=data.follower_id/>
            <FieldSelect
                name="status"
                options=vec![("0", "Pending"), ("1", "Accepted"), ("2", "Declined")]
                value=data.status.to_string()
            />
            <div>
                <SubmitButton loading=action.pending() label="Update Follower"/>
            </div>
        </ActionForm>
    }
}
