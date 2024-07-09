use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::component::button::SubmitButton;
use crate::component::input::TextInput;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{auth::model::User, auth::service::AuthService, setup::get_pool};

#[server(endpoint = "password-reset")]
pub async fn password_reset_request(email: String) -> Result<(), ServerFnError> {
    let pool = get_pool()?;
    User::validate_email(&email)?;
    let user = User::get_by_email(&pool, &email).await?;
    if let Some(user) = user {
        AuthService::send_password_reset_email(user.id, &user.name, &user.email).await?;
    }
    leptos_axum::redirect("/password-reset/email-sent");
    Ok(())
}

#[component]
pub fn PasswordResetRequestPage() -> impl IntoView {
    let action = Action::<PasswordResetRequest, _>::server();
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || extract_other_errors(action_value, &["email"]);
    let non_field_errors = move || get_non_field_errors(action_value);

    view! {
        <Title text="Password Reset"/>
        <main class="p-4 lg:p-8">
            <div class="p-4 mx-auto max-w-md bg-white border shadow-md">
                <h1 class="mb-4 text-xl font-bold">"Password Reset"</h1>

                <div class="mb-4 text-red-500 font-bold">{action_error}</div>
                <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>

                <p class="mb-4">"Enter your email address below to reset your password:"</p>
                <ActionForm action>
                    <TextInput
                        action_value
                        name="email"
                        input_type="email"
                        placeholder="Enter your email address"
                    />

                    <SubmitButton loading=action_loading label="Continue"/>
                </ActionForm>
            </div>
        </main>
    }
}
