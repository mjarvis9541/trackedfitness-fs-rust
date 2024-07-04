use leptos::*;
use leptos_meta::*;

#[component]
pub fn PasswordResetCompletePage() -> impl IntoView {
    view! {
        <Title text="Password Reset Completed"/>
        <main class="p-4 lg:p-8">
            <div class="p-4 mx-auto max-w-md bg-white border shadow-md">
                <h1 class="mb-4 text-xl font-bold">"Password Reset Completed"</h1>
                <p class="mb-4">"Your password has been successfully reset."</p>
                <p class="mb-4">"You can now use your new password to login."</p>
                <p>
                    <a class="text-blue-500 hover:underline" href="/login">
                        "Log in"
                    </a>
                </p>
            </div>
        </main>
    }
}
