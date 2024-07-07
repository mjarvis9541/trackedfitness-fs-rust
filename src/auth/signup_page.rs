use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::component::button::SubmitButton;
use crate::component::input::TextInput;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{
    auth::model::User, auth::privacy_level::PrivacyLevel, auth::service::AuthService,
    setup::get_pool,
};

#[server]
pub async fn signup(
    name: String,
    email: String,
    password: String,
    code: String,
) -> Result<(), ServerFnError> {
    let pool = get_pool()?;
    User::validate_signup(&name, &email, &password, &code)?;
    let default_privacy_level: i32 = PrivacyLevel::Followers.into();
    let username = User::generate_username_from_email(&email)?;
    let user = User::create(
        &pool,
        &name,
        &username,
        &password,
        &email,
        false,
        false,
        false,
        false,
        default_privacy_level,
    )
    .await?;
    AuthService::send_activation_email(user.id, &user.name, &user.email).await?;
    leptos_axum::redirect("/signup/email-sent");
    Ok(())
}

#[component]
pub fn SignupPage() -> impl IntoView {
    let action = Action::<Signup, _>::server();
    // let auth_context = expect_context::<crate::app::AuthContext>().0;
    // create_render_effect(move |_| {
    //     auth_context.with(|value| {
    //         if *value {
    //             let navigate = use_navigate();
    //             navigate("/", Default::default())
    //         }
    //     })
    // });
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
        <Title text="Sign up"/>
        <main class="p-4 lg:p-8">
            <div class="p-4 mx-auto max-w-md bg-white border shadow-md">
                <h1 class="mb-4 text-xl font-bold">"Sign up"</h1>
                <div class="mb-4 text-red-500 font-bold">{action_error}</div>
                <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>
                <ActionForm action>
                    <TextInput
                        name="code"
                        input_type="password"
                        label="Early access code"
                        autocomplete="new-password"
                        placeholder="Enter your early access code"
                        action_value
                    />
                    <TextInput
                        name="name"
                        label="Full name"
                        placeholder="Enter your full name"
                        action_value
                    />
                    <TextInput
                        name="email"
                        label="Email address"
                        placeholder="Enter your email address"
                        input_type="email"
                        action_value
                    />
                    <TextInput
                        name="password"
                        input_type="password"
                        autocomplete="new-password"
                        placeholder="Enter your password"
                        action_value
                    />
                    <SubmitButton loading=action_loading label="Continue"/>
                </ActionForm>

                <p class="mt-6 text-sm text-gray-600">
                    "By signing up, you agree to our "
                    <a href="/privacy-policy" class="text-blue-500 hover:underline">
                        "Privacy Policy"
                    </a> " and " <a href="/terms-of-service" class="text-blue-500 hover:underline">
                        "Terms of Service"
                    </a> "."
                </p>
                <p class="mt-4 text-gray-600">
                    "Already have an account? "
                    <a href="/login" class="text-blue-500 hover:underline">
                        "Log in"
                    </a>
                </p>
                <p class="mt-2 text-gray-600">
                    <a href="/signup/resend-email" class="text-blue-500 hover:underline">
                        "Help"
                    </a>
                </p>

            </div>
        </main>
    }
}
