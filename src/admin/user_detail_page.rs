use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use uuid::Uuid;

use crate::auth::model::User;
use crate::component::button::SubmitButton;
use crate::component::checkbox::CheckboxInput;
use crate::component::input::TextInput;
use crate::component::select::FieldSelect;
use crate::component::template::{ErrorComponent, LoadingComponent};
use crate::util::datetime::format_datetime;
use crate::util::param::UuidParam;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{auth::service::extract_superuser_from_request, error::Error, setup::get_pool};

#[server]
pub async fn get_admin_user_detail(id: Uuid) -> Result<User, ServerFnError> {
    extract_superuser_from_request()?;
    let pool = get_pool()?;
    let query = User::get_by_id(&pool, id).await?.ok_or(Error::NotFound)?;
    Ok(query)
}

#[server]
pub async fn admin_user_update(
    id: Uuid,
    username: String,
    name: String,
    email: String,
    email_verified: bool,
    is_active: bool,
    is_staff: bool,
    is_superuser: bool,
    privacy_level: i32,
) -> Result<(), ServerFnError> {
    extract_superuser_from_request()?;
    let pool = get_pool()?;
    User::update(
        &pool,
        id,
        &name,
        &username,
        Some(email),
        Some(email_verified),
        Some(is_active),
        Some(is_staff),
        Some(is_superuser),
        privacy_level,
    )
    .await?;
    Ok(())
}

#[server]
pub async fn admin_user_password_change(
    id: Uuid,
    new_password: String,
) -> Result<(), ServerFnError> {
    extract_superuser_from_request()?;
    let pool = get_pool()?;
    User::update_password(&pool, id, &new_password).await?;
    Ok(())
}

#[component]
pub fn AdminUserDetailPage() -> impl IntoView {
    let params = use_params::<UuidParam>();
    let id = move || params.with(|p| p.as_ref().map(|p| p.id).unwrap_or_default());

    let action_password_change = Action::<AdminUserPasswordChange, _>::server();
    let action = Action::<AdminUserUpdate, _>::server();

    let username = RwSignal::new(String::new());
    let resource = Resource::new(
        move || {
            (
                id(),
                action.version().get(),
                action_password_change.version().get(),
            )
        },
        |(id, ..)| get_admin_user_detail(id),
    );
    let response = move || {
        resource.and_then(|data| {
            username.update(|v| *v = data.username.clone());
            view! { <UserDetailComponent data=data.clone()/> }
        })
    };
    let form_response =
        move || resource.and_then(|data| view! { <AdminUserUpdateForm data=data.clone() action/> });
    let password_form_response = move || {
        resource.and_then(|data| view! { <AdminUserPasswordChangeForm id=data.id.clone() action=action_password_change/> })
    };
    let view_on_site_url = move || username.with(|username| format!("/users/{}", username));
    view! {
        <Title text="Admin User Detail"/>
        <main class="p-4">
            <div class="grid grid-cols-4 gap-4 md:grid-cols-8 lg:grid-cols-12">
                <div class="col-span-4">
                    <div class="p-4 mb-4 bg-white border shadow-sm">
                        <Transition fallback=LoadingComponent>
                            <h2 class="mb-4 text-xl font-bold">"User Detail"</h2>
                            <ErrorBoundary fallback=|errors| {
                                view! { <ErrorComponent errors/> }
                            }>{response}</ErrorBoundary>
                        </Transition>

                        <a href=view_on_site_url class="block mt-4 text-blue-500 hover:underline">
                            "View on Site"
                        </a>

                    </div>
                </div>
                <div class="col-span-4">
                    <div class="p-4 mb-4 bg-white border shadow-sm">
                        <h2 class="mb-4 text-xl font-bold">"User Update"</h2>
                        <Transition fallback=LoadingComponent>
                            <ErrorBoundary fallback=|errors| {
                                view! { <ErrorComponent errors/> }
                            }>{form_response}</ErrorBoundary>
                        </Transition>
                    </div>
                </div>
                <div class="col-span-4">
                    <div class="p-4 mb-4 bg-white border shadow-sm">
                        <h2 class="mb-4 text-xl font-bold">"Change Password"</h2>
                        <Transition fallback=LoadingComponent>
                            <ErrorBoundary fallback=|errors| {
                                view! { <ErrorComponent errors/> }
                            }>{password_form_response}</ErrorBoundary>
                        </Transition>
                    </div>
                </div>
            </div>
        </main>
    }
}

#[component]
pub fn AdminUserUpdateForm(
    data: User,
    action: Action<AdminUserUpdate, Result<(), ServerFnError>>,
) -> impl IntoView {
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error =
        move || extract_other_errors(action_value, &["non_field_errors", "new_password"]);
    let non_field_errors = move || get_non_field_errors(action_value);
    view! {
        <div class="mb-4 text-red-500 font-bold">{action_error}</div>
        <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>
        <ActionForm action>
            <input type="hidden" name="id" value=data.id.to_string()/>
            <TextInput action_value name="name" value=data.name/>
            <TextInput action_value name="username" value=data.username/>
            <TextInput action_value name="email" input_type="email" value=data.email/>
            <CheckboxInput name="email_verified" checked=data.email_verified/>
            <CheckboxInput name="is_active" checked=data.is_active/>
            <CheckboxInput name="is_staff" checked=data.is_staff/>
            <CheckboxInput name="is_superuser" checked=data.is_superuser/>
            <FieldSelect
                name="privacy_level"
                options=vec![
                    ("0", "N/A - All users can view your profile"),
                    ("1", "Public - All users can view your profile"),
                    ("2", "Followers Only - Only followers can view your profile"),
                    ("3", "Private - No users can view your profile"),
                ]

                value=data.privacy_level.to_string()
            />
            <SubmitButton loading=action_loading label="Update User"/>
        </ActionForm>
    }
}

#[component]
pub fn AdminUserPasswordChangeForm(
    id: Uuid,
    action: Action<AdminUserPasswordChange, Result<(), ServerFnError>>,
) -> impl IntoView {
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error =
        move || extract_other_errors(action_value, &["non_field_errors", "new_password"]);
    let non_field_errors = move || get_non_field_errors(action_value);
    view! {
        <div class="mb-4 text-red-500 font-bold">{action_error}</div>
        <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>
        <ActionForm action>
            <input type="hidden" name="id" value=id.to_string()/>
            <TextInput
                action_value
                label="New password"
                name="new_password"
                input_type="password"
                autocomplete="new-password"
            />
            <SubmitButton loading=action_loading label="Update Password"/>
        </ActionForm>
    }
}

#[component]
pub fn UserDetailComponent(data: User) -> impl IntoView {
    let created_at = format_datetime(&Some(data.created_at));
    let updated_at = format_datetime(&data.updated_at);
    let last_login = format_datetime(&data.last_login);
    view! {
        <table class="overflow-hidden w-full border-collapse">
            <tbody>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Name"</th>
                    <td class="p-2 w-1/2 text-right border">{data.name}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Username"</th>
                    <td class="p-2 w-1/2 text-right border">{data.username}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Email"</th>
                    <td class="p-2 w-1/2 text-right border">{data.email}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Email Verified"</th>
                    <td class="p-2 w-1/2 text-right border">{data.email_verified}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Privacy Level"</th>
                    <td class="p-2 w-1/2 text-right border">{data.privacy_level.to_string()}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Active"</th>
                    <td class="p-2 w-1/2 text-right border">{data.is_active}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Staff"</th>
                    <td class="p-2 w-1/2 text-right border">{data.is_staff}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Superuser"</th>
                    <td class="p-2 w-1/2 text-right border">{data.is_superuser}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Last Login"</th>
                    <td class="p-2 w-1/2 text-right border">{last_login}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Created"</th>
                    <td class="p-2 w-1/2 text-right border">{created_at}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Updated"</th>
                    <td class="p-2 w-1/2 text-right border">{updated_at}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Updated"</th>
                    <td class="p-2 w-1/2 text-right border">{data.id.to_string()}</td>
                </tr>
            </tbody>
        </table>
    }
}
