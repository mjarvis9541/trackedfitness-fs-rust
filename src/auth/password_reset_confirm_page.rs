use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::component::button::SubmitButton;
use crate::component::input::TextInputImproved;
use crate::util::param::extract_param;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{
    auth::{
        model::User,
        token::{JwtManager, TokenType},
    },
    setup::get_pool,
};

#[server(endpoint = "password-reset-confirm")]
pub async fn password_reset_confirm(token: String, password: String) -> Result<(), ServerFnError> {
    let token = JwtManager::validate_token(&token, TokenType::PasswordReset)?;
    let pool = get_pool()?;

    User::validate_password(&password)?;
    User::update_password(&pool, token.user_id, &password).await?;

    leptos_axum::redirect("/password-reset/complete");
    Ok(())
}

#[component]
pub fn PasswordResetConfirmPage() -> impl IntoView {
    let query = use_query_map();
    let token = move || extract_param(&query, "token");

    let action = Action::<PasswordResetConfirm, _>::server();
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || extract_other_errors(action_value, &["password"]);
    let non_field_errors = move || get_non_field_errors(action_value);

    // let handle_submit = move |ev: ev::SubmitEvent| {
    //     ev.prevent_default();
    //     let data = PasswordReset::from_event(&ev);
    //     if let Ok(mut data) = data {
    //         data.token = Some(token());
    //         action.dispatch(data)
    //     }
    // };

    view! {
        <Title text="Enter New Password"/>

        <main class="p-4 lg:p-8">
            <div class="p-4 mx-auto max-w-md bg-white border shadow-md">
                <h1 class="mb-4 text-xl font-bold">"Enter New Password"</h1>

                <p class="mb-4">"Please enter your new password below."</p>

                <div class="mb-4 text-red-500 font-bold">{action_error}</div>
                <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>

                <ActionForm action>
                    <input type="hidden" name="token" value=token/>
                    <TextInputImproved
                        name="password"
                        input_type="password"
                        autocomplete="new-password"
                        action_value
                    />
                    <SubmitButton loading=action_loading label="Update Password"/>
                </ActionForm>

                <p class="mt-4 mb-2">
                    <A href="/password-reset" class="text-blue-500 hover:underline">
                        "Resend email"
                    </A>
                </p>
            </div>
        </main>
    }
}
