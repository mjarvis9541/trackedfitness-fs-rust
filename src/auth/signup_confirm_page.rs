use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::component::button::SubmitButton;
use crate::component::input::ValidatedInput;
use crate::error_extract::{extract_error_message, process_non_field_errors};
use crate::util::param::extract_param;

#[cfg(feature = "ssr")]
use crate::{
    auth::{
        cookie::set_jwt_cookie,
        model::User,
        token::{JwtManager, TokenType},
    },
    setup::get_pool,
};

#[server(endpoint = "signup-confirm")]
pub async fn signup_confirm(token: String) -> Result<(), ServerFnError> {
    let token = JwtManager::validate_token(&token, TokenType::Activation)?;
    let pool = get_pool()?;
    let user = User::activate(&pool, token.user_id).await?;
    let token = JwtManager::generate_auth_token(
        user.id,
        &user.username,
        user.is_active,
        user.is_staff,
        user.is_superuser,
    )?;
    set_jwt_cookie(&token)?;
    leptos_axum::redirect(&format!("/users/{}/setup", user.username));
    Ok(())
}

#[component]
pub fn SignupConfirmPage(
    action: Action<SignupConfirm, Result<(), ServerFnError>>,
) -> impl IntoView {
    let query = use_query_map();
    let token = move || extract_param(&query, "token");

    let error = move || extract_error_message(&action);
    let non_field_errors = move || process_non_field_errors(error);
    let error = Signal::derive(error);

    view! {
        <Title text="Enter Activation Code"/>
        <main class="p-4 lg:p-8">

            <div class="p-4 mx-auto max-w-md bg-white border shadow-md">
                <h1 class="mb-4 text-xl font-bold">"Enter Activation Code"</h1>
                {non_field_errors}
                {error}
                <p class="mb-4">"Enter your activation code below to activate your account."</p>
                <ActionForm action>
                    <ValidatedInput error name="token" label="Activation code" value=token()/>
                    <SubmitButton loading=action.pending() label="Activate Account"/>
                </ActionForm>

                <div class="pt-4">
                    <p>
                        <A href="/signup/resend-email" class="text-blue-500 hover:underline">
                            "Resend activation email"
                        </A>
                    </p>
                </div>

            </div>
        </main>
    }
}
