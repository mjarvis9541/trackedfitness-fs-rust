use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::component::button::SubmitButton;
use crate::component::input::TextInput;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{
    auth::model::User, auth::service::get_request_user, auth::service::AuthService, error::Error,
    setup::get_pool,
};

#[server(endpoint = "email-change-request")]
pub async fn email_change_request(email: String) -> Result<(), ServerFnError> {
    let request_user = get_request_user()?;
    let pool = get_pool()?;

    User::validate_email(&email)?;
    if User::get_by_email(&pool, &email).await?.is_some() {
        use_context::<leptos_axum::ResponseOptions>()
            .map(|res| res.set_status(http::StatusCode::BAD_REQUEST));
        return Err(ServerFnError::new("This email is taken"));
    }

    let user = User::get_by_username(&pool, &request_user.username)
        .await?
        .ok_or(Error::NotFound)?;

    AuthService::send_email_change_email(user.id, &user.name, &email).await?;

    leptos_axum::redirect("/settings/change-change/email-sent");
    Ok(())
}

#[component]
pub fn EmailChangeRequestPage() -> impl IntoView {
    let action = Action::<EmailChangeRequest, _>::server();
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || {
        extract_other_errors(
            action_value,
            &["non_field_errors", "code", "name", "email", "password"],
        )
    };
    let non_field_errors = move || get_non_field_errors(action_value);

    view! {
        <Title text="Change Email"/>
        <main class="p-4 bg-white">
            <div class="max-w-sm">
                <h1 class="mb-2 text-base font-bold">"Change Email"</h1>
                <p class="mb-4">"Update your email address below."</p>
                <div class="mb-4 text-red-500 font-bold">{action_error}</div>
                <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>
                <ActionForm action>
                    <TextInput
                        name="email"
                        label="New email address"
                        placeholder="Enter your new email address"
                        input_type="email"
                        action_value
                    />
                    <SubmitButton loading=action_loading label="Continue"/>
                </ActionForm>
            </div>
        </main>
    }
}
