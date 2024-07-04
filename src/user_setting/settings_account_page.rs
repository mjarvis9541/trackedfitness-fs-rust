use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::auth::model::User;
use crate::component::button::SubmitButton;
use crate::component::input::ValidatedInput;
use crate::component::select::{FieldSelect, USER_PRIVACY_FORM_OPTIONS};
use crate::component::template::{ErrorComponent, LoadingComponent};
use crate::error_extract::{extract_error_message, process_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, error::Error, setup::get_pool};

#[server]
async fn get_request_user_detail() -> Result<User, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    let user = User::get_by_username(&pool, &user.username)
        .await?
        .ok_or(Error::NotFound)?;
    Ok(user)
}

#[server]
async fn request_user_update(
    name: String,
    username: String,
    privacy_level: i32,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    User::update(
        &pool,
        user.id,
        &name,
        &username,
        None,
        None,
        None,
        None,
        None,
        privacy_level,
    )
    .await?;
    Ok(())
}

#[component]
pub fn UserAccountSettingsPage() -> impl IntoView {
    let action = Action::<RequestUserUpdate, _>::server();
    let resource = Resource::new(
        move || action.version().get(),
        |_| get_request_user_detail(),
    );
    let response = move || {
        resource.and_then(|data| view! { <UserSettingsUpdateForm data=data.clone() action/> })
    };
    view! {
        <Title text="Account Settings"/>
        <div class="p-4 bg-white border">
            <h1 class="mb-2 text-base font-bold">"Account Settings"</h1>
            <p class="mb-4">"This is how others will see you on the site."</p>
            <hr class="mb-8"/>
            <Transition fallback=LoadingComponent>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors/> }
                }>{response}</ErrorBoundary>
            </Transition>
        </div>
    }
}

#[component]
fn UserSettingsUpdateForm(
    data: User,
    action: Action<RequestUserUpdate, Result<(), ServerFnError>>,
) -> impl IntoView {
    let error = move || extract_error_message(&action);
    let non_field_errors = move || process_non_field_errors(error);
    let error = Signal::derive(error);
    view! {
        <div class="max-w-sm">
            {non_field_errors} <ActionForm action>
                <ValidatedInput error name="name" value=data.name/>
                <ValidatedInput error name="username" value=data.username/>
                <ValidatedInput
                    error
                    name="email"
                    input_type="email"
                    value=data.email
                    disabled=true
                />
                <FieldSelect
                    name="privacy_level"
                    options=&USER_PRIVACY_FORM_OPTIONS
                    value=data.privacy_level.to_string()
                />
                <SubmitButton loading=action.pending() label="Update Settings"/>
            </ActionForm>
        </div>
    }
}
