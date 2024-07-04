use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn SignupDonePage() -> impl IntoView {
    view! {
        <Title text="Email Sent"/>
        <main class="p-4 lg:p-8">
            <div class="p-4 mx-auto max-w-md bg-white border shadow-md">
                <h1 class="mb-4 text-xl font-bold">"Email Sent"</h1>
                <div class="space-y-4">
                    <p>"A verification email has been sent to your email address."</p>
                    <p>
                        "Please check your inbox and follow the instructions to complete the sign up process."
                    </p>
                    <p>
                        <A href="/signup/confirm" class="text-blue-500 hover:underline">
                            "Enter activation code"
                        </A>
                    </p>
                </div>
            </div>
        </main>
    }
}
