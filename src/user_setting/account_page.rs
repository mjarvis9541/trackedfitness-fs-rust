use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::auth::model::User;
use crate::auth::privacy_level::PrivacyLevel;
use crate::component::button::SubmitButton;
use crate::component::input::TextInput;
use crate::component::select::FieldSelect;
use crate::component::template::{ErrorComponent, LoadingComponent};
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

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
            <Transition fallback=LoadingComponent>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors/> }
                }>{response}</ErrorBoundary>
            </Transition>
        </div>
    }
}

#[component]
pub fn UserSettingsUpdateForm(
    data: User,
    action: Action<RequestUserUpdate, Result<(), ServerFnError>>,
) -> impl IntoView {
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || {
        extract_other_errors(
            action_value,
            &[
                "non_field_errors",
                "name",
                "username",
                "email",
                "privacy_level",
            ],
        )
    };
    let non_field_errors = move || get_non_field_errors(action_value);

    view! {
        <div class="max-w-sm">
            <div class="mb-4 text-red-500 font-bold">{action_error}</div>
            <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>
            <ActionForm action>
                <TextInput action_value name="name" value=data.name/>
                <TextInput action_value name="username" value=data.username/>
                <TextInput
                    action_value
                    name="email"
                    input_type="email"
                    value=data.email
                    disabled=true
                />
                <FieldSelect
                    name="privacy_level"
                    options=PrivacyLevel::to_form_options()
                    value=data.privacy_level.to_form_value()
                />
                <SubmitButton loading=action_loading label="Update Settings"/>
            </ActionForm>
        </div>
    }
}
