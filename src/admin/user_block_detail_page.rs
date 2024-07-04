use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use uuid::Uuid;

use super::user_select::UserSelect;
use crate::component::button::SubmitButton;
use crate::component::template::{ErrorComponent, LoadingComponent};
use crate::error_extract::{extract_error_message, process_non_field_errors};
use crate::user_block::model::UserBlock;
use crate::util::datetime::format_datetime;
use crate::util::param::UuidParam;
use crate::util::text::capitalize_and_replace;

#[server]
async fn admin_user_block_update(
    id: Uuid,
    blocker_id: Uuid,
    blocked_id: Uuid,
    blocked_status: i32,
) -> Result<(), ServerFnError> {
    crate::auth::service::extract_superuser_from_request()?;
    let pool = crate::setup::get_pool()?;
    UserBlock::update(&pool, id, blocker_id, blocked_id, blocked_status).await?;
    Ok(())
}

#[server]
async fn get_admin_user_block_detail(id: Uuid) -> Result<UserBlock, ServerFnError> {
    crate::auth::service::extract_superuser_from_request()?;
    let pool = crate::setup::get_pool()?;
    let object = UserBlock::get_object_or_404(&pool, id).await?;
    Ok(object)
}

#[component]
pub fn AdminUserBlockDetailPage() -> impl IntoView {
    let action = Action::<AdminUserBlockUpdate, _>::server();
    let params = use_params::<UuidParam>();
    let id = move || params.with(|q| q.as_ref().map(|q| q.id).unwrap_or_default());
    let resource = Resource::new(
        move || (id(), action.version().get()),
        |(id, _)| get_admin_user_block_detail(id),
    );
    let response = move || {
        resource.and_then(|data| view! { <AdminUserBlockDetailComponent data=data.clone()/> })
    };
    let form_response = move || {
        resource.and_then(|data| view! { <AdminUserBlockUpdateForm data=data.clone() action/> })
    };
    let error = move || extract_error_message(&action);
    let non_field_errors = move || process_non_field_errors(error);
    view! {
        <Title text="Admin - Blocked User"/>
        <main class="grid grid-cols-4 gap-4 p-4 md:grid-cols-8 lg:grid-cols-12">
            <section class="col-span-4">
                <div class="p-4 bg-white border">
                    <h1 class="mb-4 text-xl font-bold">"Admin - Blocked User"</h1>
                    <Transition fallback=LoadingComponent>
                        <ErrorBoundary fallback=|errors| {
                            view! { <ErrorComponent errors/> }
                        }>{response}</ErrorBoundary>
                    </Transition>
                </div>
            </section>
            <section class="col-span-4">
                <div class="p-4 mb-4 bg-white">
                    <h2 class="mb-4 text-xl font-bold">"Update Blocked User"</h2>
                    {error}
                    {non_field_errors}
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
pub fn AdminUserBlockDetailComponent(data: UserBlock) -> impl IntoView {
    let created_at = format_datetime(&Some(data.blocked_at));
    let updated_at = format_datetime(&data.unblocked_at);
    view! {
        <table class="w-full border-collapse">
            <tbody>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Blocking User"</th>
                    <td class="p-2 w-1/2 text-right border">{data.blocker_username}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Blocked User"</th>
                    <td class="p-2 w-1/2 text-right border">{data.blocked_username}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Status"</th>
                    <td class="p-2 w-1/2 text-right border">{data.blocked_status.to_string()}</td>
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
pub fn AdminUserBlockUpdateForm(
    data: UserBlock,
    action: Action<AdminUserBlockUpdate, Result<(), ServerFnError>>,
) -> impl IntoView {
    view! {
        <ActionForm action>
            <input type="hidden" name="id" value=data.id.to_string()/>
            <UserSelect name="blocker_id" label="blocker" selected=data.blocker_id/>
            <UserSelect name="blocked_id" label="blocked" selected=data.blocked_id/>
            <FieldSelect
                name="blocked_status"
                options=vec![("0", "Unblocked"), ("1", "Blocked")]
                value=data.blocked_status.to_string()
            />
            <div>
                <SubmitButton loading=action.pending() label="Update Blocked User"/>
            </div>
        </ActionForm>
    }
}

#[component]
pub fn FieldSelect(
    name: &'static str,
    #[prop(default = name)] label: &'static str,
    #[prop(optional, into)] value: String,
    options: Vec<(&'static str, &'static str)>,
) -> impl IntoView {
    view! {
        <label class="block mb-4">
            <span class="block mb-1 text-sm font-bold">{capitalize_and_replace(label)}</span>
            <select
                name=name
                class="block py-2.5 px-3 w-full bg-white rounded border focus:border-blue-500 focus:ring-2 focus:ring-blue-500 focus:outline-none"
            >
                {move || {
                    options
                        .clone()
                        .into_iter()
                        .map(|(option_value, label)| {
                            let selected_value = value.clone();
                            view! {
                                <option
                                    value=option_value
                                    prop:selected=move || *option_value == selected_value
                                >
                                    {label}
                                </option>
                            }
                        })
                        .collect_view()
                }}

            </select>
        </label>
    }
}
