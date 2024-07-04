use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::component::button::SubmitButton;
use crate::component::input::TextInputImproved;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{auth::model::User, auth::service::AuthService, error::Error, setup::get_pool};

#[server(endpoint = "signup-resend")]
async fn signup_request_resend(email: String) -> Result<(), ServerFnError> {
    let pool = get_pool()?;

    User::validate_email(&email)?;

    let user = User::get_by_email(&pool, &email)
        .await?
        .ok_or(Error::NotFound)?;

    user.ensure_email_not_verified()?;
    user.ensure_account_not_active()?;

    AuthService::send_activation_email(user.id, &user.name, &user.email).await?;

    leptos_axum::redirect("/signup/email-sent");
    Ok(())
}

#[component]
pub fn SignupResendPage() -> impl IntoView {
    let action = Action::<SignupRequestResend, _>::server();

    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || extract_other_errors(action_value, &["name"]);
    let non_field_errors = move || get_non_field_errors(action_value);

    view! {
        <Title text="Resend Activation Email"/>
        <main class="p-4 lg:p-8">

            <div class="p-4 mx-auto max-w-md bg-white border shadow-md">
                <h1 class="mb-4 text-xl font-bold">"Resend Activation Email"</h1>

                <p class="mb-4">
                    "If you've not received an activation code via email, enter your email address and we'll send you a new one."
                </p>
                <div class="mb-4 text-red-500 font-bold">{action_error}</div>
                <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>

                <ActionForm action>
                    <TextInputImproved
                        name="email"
                        label="Email address"
                        placeholder="Enter your email address"
                        input_type="email"
                        action_value
                    />
                    <SubmitButton loading=action_loading label="Continue"/>
                </ActionForm>
            </div>
        </main>
    }
}
