use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::component::button::SubmitButton;
use crate::component::input::TextInput;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{auth::model::User, auth::service::get_request_user, error::Error, setup::get_pool};

#[server(endpoint = "password-change")]
pub async fn password_change(
    old_password: String,
    new_password: String,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    User::validate_password_change(&old_password, &new_password)?;
    let user = User::get_by_username(&pool, &user.username)
        .await?
        .ok_or(Error::NotFound)?;
    user.verify_password(&old_password)?;
    User::update_password(&pool, user.id, &new_password).await?;
    leptos_axum::redirect("/settings");
    Ok(())
}

#[component]
pub fn PasswordUpdatePage() -> impl IntoView {
    let action = Action::<PasswordChange, _>::server();
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || {
        extract_other_errors(
            action_value,
            &["non_field_errors", "old_password", "new_password"],
        )
    };
    let non_field_errors = move || get_non_field_errors(action_value);

    view! {
        <Title text="Change Password"/>
        <main class="p-4 bg-white">
            <div class="max-w-sm">
                <h1 class="mb-2 text-base font-bold">"Change Password"</h1>
                <p class="mb-4">"Update your password below."</p>
                <div class="mb-4 text-red-500 font-bold">{action_error}</div>
                <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>
                <ActionForm action>
                    <TextInput
                        action_value
                        label="Old password"
                        name="old_password"
                        input_type="password"
                        autocomplete="new-password"
                        placeholder="Enter your old password"
                    />
                    <TextInput
                        action_value
                        label="New password"
                        name="new_password"
                        input_type="password"
                        autocomplete="new-password"
                        placeholder="Enter your new password"
                    />
                    <SubmitButton loading=action_loading label="Update Password"/>
                </ActionForm>
            </div>
        </main>
    }
}
