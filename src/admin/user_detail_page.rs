use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use uuid::Uuid;

use crate::auth::model::User;
use crate::component::button::SubmitButton;
use crate::component::checkbox::CheckboxInput;
use crate::component::input::ValidatedInput;
use crate::component::select::{FieldSelect, USER_PRIVACY_FORM_OPTIONS};
use crate::component::template::{ErrorComponent, LoadingComponent};
use crate::error_extract::{extract_error_message, process_non_field_errors};
use crate::util::datetime::format_datetime;
use crate::util::param::UuidParam;

#[cfg(feature = "ssr")]
use crate::error::Error;

#[server]
pub async fn get_admin_user_detail(id: Uuid) -> Result<User, ServerFnError> {
    crate::auth::service::extract_superuser_from_request()?;
    let pool = expect_context::<sqlx::PgPool>();
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
    crate::auth::service::extract_superuser_from_request()?;
    let pool = expect_context::<sqlx::PgPool>();
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

#[component]
pub fn AdminUserDetailPage() -> impl IntoView {
    let action = Action::<AdminUserUpdate, _>::server();
    let error = move || extract_error_message(&action);
    let non_field_errors = move || process_non_field_errors(error);

    let params = use_params::<UuidParam>();
    let id = move || params.with(|p| p.as_ref().map_or_else(|_| Uuid::default(), |p| p.id));

    let resource = Resource::new(
        move || (id(), action.version().get()),
        |(id, _)| get_admin_user_detail(id),
    );
    let response =
        move || resource.and_then(|data| view! { <UserDetailComponent data=data.clone()/> });
    let form_response =
        move || resource.and_then(|data| view! { <AdminUserUpdateForm data=data.clone() action/> });

    view! {
        <Title text="Admin User Detail"/>
        <main class="p-4">

            <div class="grid grid-cols-4 gap-4 md:grid-cols-8 lg:grid-cols-12">

                <div class="col-span-4">
                    <div class="p-4 mb-4 bg-white">
                        <Transition fallback=LoadingComponent>
                            <ErrorBoundary fallback=|errors| {
                                view! { <ErrorComponent errors/> }
                            }>{response}</ErrorBoundary>
                        </Transition>
                    </div>
                </div>

                <div class="col-span-4">
                    <div class="p-4 bg-white border">
                        <h2 class="mb-4 text-base font-bold">"Update User"</h2>
                        {error}
                        {non_field_errors}
                        <Transition fallback=LoadingComponent>
                            <ErrorBoundary fallback=|errors| {
                                view! { <ErrorComponent errors/> }
                            }>{form_response}</ErrorBoundary>
                        </Transition>
                    </div>
                </div>
                <div class="col-span-4">
                    <div class="p-4 bg-white border"></div>
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
    let error = move || extract_error_message(&action);
    let error = Signal::derive(error);

    view! {
        <ActionForm action>
            <input type="hidden" name="id" value=data.id.to_string()/>
            <ValidatedInput error name="name" value=data.name/>
            <ValidatedInput error name="username" value=data.username/>
            <ValidatedInput error name="email" input_type="email" value=data.email/>
            <CheckboxInput name="email_verified" checked=data.email_verified/>
            <CheckboxInput name="is_active" checked=data.is_active/>
            <CheckboxInput name="is_staff" checked=data.is_staff/>
            <CheckboxInput name="is_superuser" checked=data.is_superuser/>
            <FieldSelect
                name="privacy_level"
                options=&USER_PRIVACY_FORM_OPTIONS
                value=data.privacy_level.to_string()
            />
            <SubmitButton loading=action.pending()/>
        </ActionForm>
    }
}

#[component]
pub fn UserDetailComponent(data: User) -> impl IntoView {
    let created_at = format_datetime(&Some(data.created_at));
    let updated_at = format_datetime(&data.updated_at);
    let last_login = format_datetime(&data.last_login);

    view! {
        <h2 class="mb-4 text-base font-bold">"User Detail"</h2>
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
