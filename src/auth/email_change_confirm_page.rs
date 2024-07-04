use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::component::button::SubmitButton;
use crate::component::input::TextInputImproved;
use crate::util::param::extract_param;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user,
    auth::{
        model::User,
        token::{JwtManager, TokenType},
    },
    setup::get_pool,
};

#[server(endpoint = "email-change-confirm")]
pub async fn email_change_confirm(token: String) -> Result<(), ServerFnError> {
    get_request_user()?;
    let pool = get_pool()?;

    let token = JwtManager::validate_token(&token, TokenType::EmailChange)?;

    User::update_email(&pool, token.user_id, &token.email).await?;

    leptos_axum::redirect("/settings/change-email/complete");
    Ok(())
}

#[component]
pub fn EmailChangeConfirmPage() -> impl IntoView {
    let query = use_query_map();
    let token = move || extract_param(&query, "token");

    let action = Action::<EmailChangeConfirm, _>::server();
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || extract_other_errors(action_value, &["name"]);
    let non_field_errors = move || get_non_field_errors(action_value);

    view! {
        <Title text="Enter Activation Code"/>
        <main class="p-4 lg:p-8">
            <div class="p-4 mx-auto max-w-md bg-white border shadow-md">
                <h1 class="mb-4 text-xl font-bold">"Enter Activation Code"</h1>

                <div class="mb-4 text-red-500 font-bold">{action_error}</div>
                <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>

                <p class="mb-4">
                    "Please enter the activation code sent to your new email address: "
                </p>

                <ActionForm action>
                    <TextInputImproved
                        action_value
                        name="token"
                        label="Activation code"
                        value=token()
                    />
                    <SubmitButton loading=action_loading label="Update Email"/>
                </ActionForm>

                <p class="mt-4 mb-2">
                    <A href="/settings/change-email" class="text-blue-500 hover:underline">
                        "Resend email"
                    </A>
                </p>

            </div>
        </main>
    }
}
