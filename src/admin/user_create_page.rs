use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use super::user_select::{get_user_select, UserSelectResource};
use crate::component::button::SubmitButton;
use crate::component::checkbox::CheckboxInput;
use crate::component::input::ValidatedInput;
use crate::component::select::{FieldSelect, USER_PRIVACY_FORM_OPTIONS};
use crate::error_extract::{extract_error_message, process_non_field_errors};

#[server]
async fn user_create(
    name: String,
    username: String,
    password: String,
    email: String,
    email_verified: bool,
    is_active: bool,
    is_staff: bool,
    is_superuser: bool,
    privacy_level: i32,
) -> Result<(), ServerFnError> {
    crate::auth::service::extract_superuser_from_request()?;
    let pool = expect_context::<sqlx::PgPool>();
    let user = crate::auth::model::User::create(
        &pool,
        &name,
        &username,
        &password,
        &email,
        email_verified,
        is_active,
        is_staff,
        is_superuser,
        privacy_level,
    )
    .await?;
    leptos_axum::redirect(&format!("/admin/users/{}", user.id));
    Ok(())
}

#[component]
pub fn AdminUserCreatePage() -> impl IntoView {
    let resource: UserSelectResource = Resource::once(get_user_select);
    provide_context(resource);

    let action = Action::<UserCreate, _>::server();

    let error = move || extract_error_message(&action);
    let non_field_errors = move || process_non_field_errors(error);
    let error = Signal::derive(error);
    view! {
        <Title text="Create User"/>

        <div class="p-4 m-4 max-w-md bg-white border">
            <h1 class="mb-4 text-base font-bold">"Create User"</h1>
            {error}
            {non_field_errors}
            <ActionForm action>
                <ValidatedInput error name="name"/>
                <ValidatedInput
                    error
                    name="email"
                    input_type="email"
                    placeholder="michael@example.com"
                />
                <ValidatedInput error name="username" autocomplete="new-password"/>
                <ValidatedInput
                    error
                    name="password"
                    input_type="password"
                    autocomplete="new-password"
                />
                <CheckboxInput name="email_verified"/>
                <CheckboxInput name="is_active"/>
                <CheckboxInput name="is_staff"/>
                <CheckboxInput name="is_superuser"/>
                <FieldSelect name="privacy_level" options=&USER_PRIVACY_FORM_OPTIONS/>
                <SubmitButton loading=action.pending() label="Create User"/>
            </ActionForm>
        </div>
    }
}
