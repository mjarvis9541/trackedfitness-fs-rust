use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::component::button::SubmitButton;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[server(endpoint = "logout")]
pub async fn logout() -> Result<(), ServerFnError> {
    crate::auth::cookie::delete_jwt_cookie()?;
    leptos_axum::redirect("/login");
    Ok(())
}

#[component]
pub fn LogoutPage(logout: Action<Logout, Result<(), ServerFnError>>) -> impl IntoView {
    // let auth_context = expect_context::<crate::app::AuthContext>().0;
    // create_render_effect(move |_| {
    //     auth_context.with(|value| {
    //         if !*value {
    //             let navigate = use_navigate();
    //             navigate("/login", Default::default())
    //         }
    //     })
    // });
    let action_value = logout.value();
    let action_error = move || extract_other_errors(action_value, &["non_field_errors"]);
    let non_field_errors = move || get_non_field_errors(action_value);
    view! {
        <Title text="Log out"/>
        <main class="p-4 lg:p-8">
            <div class="p-4 mx-auto max-w-md bg-white border shadow-md">
                <h1 class="mb-4 text-xl font-bold">"Log out"</h1>
                <p class="mb-4">"Are you sure you wish to log out?"</p>
                <div class="mb-4 text-red-500 font-bold">{action_error}</div>
                <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>
                <ActionForm action=logout>
                    <SubmitButton loading=logout.pending() label="Log out"/>
                </ActionForm>
            </div>
        </main>
    }
}
