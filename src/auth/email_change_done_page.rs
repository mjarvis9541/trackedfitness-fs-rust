use leptos::*;
use leptos_meta::*;

#[component]
pub fn EmailChangeRequestDonePage() -> impl IntoView {
    view! {
        <Title text="Email Sent"/>
        <main class="p-4 lg:p-8">
            <div class="p-4 mx-auto max-w-md bg-white border shadow-md">
                <h1 class="mb-4 text-xl font-bold">"Email Sent"</h1>
                <p class="mb-4">"A confirmation email has been sent to your new email address."</p>

                <p class="mb-4">
                    "Please check your inbox and follow the instructions to complete the email change process."
                </p>
            </div>
        </main>
    }
}
