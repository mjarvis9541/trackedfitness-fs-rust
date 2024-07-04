use leptos::*;
use leptos_meta::Title;

#[component]
pub fn PasswordResetRequestDonePage() -> impl IntoView {
    view! {
        <Title text="Password Reset Request Sent"/>
        <main class="p-4 lg:p-8">
            <div class="p-4 mx-auto max-w-md bg-white border shadow-md">
                <h1 class="mb-4 text-xl font-bold">"Password Reset Request Sent"</h1>
                <p class="mb-4">"A password reset link has been sent to your email address."</p>
                <p class="mb-4">
                    "Please check your inbox and follow the instructions to reset your password."
                </p>
                <p>
                    <a href="/login" class="text-blue-500 hover:underline">
                        "Log in"
                    </a>
                </p>
            </div>
        </main>
    }
}
