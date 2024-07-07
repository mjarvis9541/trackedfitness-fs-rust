use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::component::button::SubmitButton;
use crate::component::input::TextInput;
use crate::util::param::extract_param;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

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

    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || extract_other_errors(action_value, &["name"]);
    let non_field_errors = move || get_non_field_errors(action_value);

    view! {
        <Title text="Enter Activation Code"/>
        <main class="p-4 lg:p-8">

            <div class="p-4 mx-auto max-w-md bg-white border shadow-md">
                <h1 class="mb-4 text-xl font-bold">"Enter Activation Code"</h1>
                <p class="mb-4">"Enter your activation code below to activate your account."</p>
                <div class="mb-4 text-red-500 font-bold">{action_error}</div>
                <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>
                <ActionForm action>
                    <TextInput action_value name="token" label="Activation code" value=token()/>
                    <SubmitButton loading=action_loading label="Activate Account"/>
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
