use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::component::button::SubmitButton;
use crate::component::input::ValidatedInput;
use crate::error_extract::{extract_error_message, process_non_field_errors};

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
    let error = move || extract_error_message(&action);
    let non_field_errors = move || process_non_field_errors(error);

    view! {
        <Title text="Change Password"/>
        <main class="p-4 bg-white">
            <div class="max-w-sm">
                <h1 class="mb-2 text-base font-bold">"Change Password"</h1>
                <p class="mb-4">"Update your password below."</p>
                <hr class="mb-8"/>

                <div
                    class="my-4 space-y-2 font-bold text-red-500"
                    class=("hidden", move || error().is_none())
                >
                    {non_field_errors}
                    {error}
                </div>
                <ActionForm action>

                    <ValidatedInput
                        name="old_password"
                        input_type="password"
                        autocomplete="new-password"
                        error=Signal::derive(error)
                    />
                    <ValidatedInput
                        name="new_password"
                        input_type="password"
                        autocomplete="new-password"
                        error=Signal::derive(error)
                    />
                    <div class="inline-block mt-6">
                        <SubmitButton loading=action.pending() label="Update Password"/>
                    </div>
                </ActionForm>
            </div>
        </main>
    }
}
