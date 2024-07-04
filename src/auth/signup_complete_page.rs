use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn SignupCompletePage() -> impl IntoView {
    view! {
        <Title text="Account Activated"/>
        <main class="p-4 lg:p-8">
            <div class="p-4 mx-auto max-w-md bg-white border shadow-md">
                <h1 class="mb-4 text-xl font-bold">"Account Activated"</h1>
                <div class="space-y-4">
                    <p>
                        "Your account has been successfully activated. You can now login to access your account."
                    </p>
                    <p>
                        <A href="/login" class="text-blue-500 hover:underline">
                            "Log in"
                        </A>
                    </p>
                </div>

            </div>
        </main>
    }
}
