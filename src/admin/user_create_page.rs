use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use super::user_select::{get_user_select, UserSelectResource};
use crate::component::button::SubmitButton;
use crate::component::checkbox::CheckboxInput;
use crate::component::input::TextInput;
use crate::component::select::FieldSelect;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

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
    let pool = crate::setup::get_pool()?;
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
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || {
        extract_other_errors(
            action_value,
            &["non_field_errors", "name", "email", "password"],
        )
    };
    let non_field_errors = move || get_non_field_errors(action_value);

    view! {
        <Title text="Create User"/>

        <div class="p-4 m-4 max-w-md bg-white border">
            <h1 class="mb-4 text-base font-bold">"Create User"</h1>
            <div class="mb-4 text-red-500 font-bold">{action_error}</div>
            <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>
            <ActionForm action>
                <TextInput action_value name="name"/>
                <TextInput
                    action_value
                    name="email"
                    input_type="email"
                    placeholder="michael@example.com"
                />
                <TextInput action_value name="username" autocomplete="new-password"/>
                <TextInput
                    action_value
                    name="password"
                    input_type="password"
                    autocomplete="new-password"
                />
                <CheckboxInput name="email_verified"/>
                <CheckboxInput name="is_active"/>
                <CheckboxInput name="is_staff"/>
                <CheckboxInput name="is_superuser"/>
                <FieldSelect
                    name="privacy_level"
                    options=vec![
                        ("0", "N/A - All users can view your profile"),
                        ("1", "Public - All users can view your profile"),
                        ("2", "Followers Only - Only followers can view your profile"),
                        ("3", "Private - No users can view your profile"),
                    ]
                />

                <SubmitButton loading=action_loading label="Create User"/>
            </ActionForm>
        </div>
    }
}
