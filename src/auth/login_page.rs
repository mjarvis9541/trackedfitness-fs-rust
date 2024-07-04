use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::component::button::SubmitButton;
use crate::component::input::TextInputImproved;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{
    auth::cookie::set_jwt_cookie, auth::model::User, auth::token::JwtManager, error::Error,
};

#[server(endpoint = "login")]
pub async fn login(email: String, password: String) -> Result<(), ServerFnError> {
    let pool = crate::setup::get_pool()?;

    User::validate_login(&email, &password)?;

    let user = User::get_by_email(&pool, &email)
        .await?
        .ok_or(Error::InvalidCredentials)?;

    user.verify_password(&password)?;
    user.ensure_account_active()?;
    user.ensure_email_verified()?;

    let token = JwtManager::generate_auth_token(
        user.id,
        &user.username,
        user.is_active,
        user.is_staff,
        user.is_superuser,
    )?;

    set_jwt_cookie(&token)?;

    leptos_axum::redirect(&format!("/users/{}", user.username));
    Ok(())
}

#[component]
pub fn LoginPage(login: Action<Login, Result<(), ServerFnError>>) -> impl IntoView {
    let action_loading = login.pending();
    let action_value = login.value();
    let action_error = move || extract_other_errors(action_value, &["email", "password"]);
    let non_field_errors = move || get_non_field_errors(action_value);

    view! {
        <Title text="Log in"/>
        <main class="p-4 lg:p-8">

            <div class="p-4 mx-auto max-w-md bg-white border shadow-md">
                <h1 class="mb-4 text-xl font-bold">"Log in"</h1>

                <div class="mb-4 text-red-500 font-bold">{action_error}</div>
                <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>
                <ActionForm action=login>
                    <TextInputImproved
                        name="email"
                        input_type="email"
                        label="Email address"
                        placeholder="Enter your email address"
                        action_value
                    />

                    <TextInputImproved
                        name="password"
                        input_type="password"
                        placeholder="Enter your password"
                        autocomplete="new-password"
                        action_value
                    />
                    <SubmitButton loading=action_loading label="Log in"/>
                </ActionForm>

                <div class="pt-4 space-y-2">
                    <p>
                        <A href="/password-reset" class="text-blue-500 hover:underline">
                            "Forgot password?"
                        </A>
                    </p>
                    <p>
                        "Need an account? " <A href="/signup" class="text-blue-500 hover:underline">
                            "Sign up"
                        </A>
                    </p>
                </div>
            </div>
        </main>
    }
}
