use leptos::*;
use leptos_meta::*;

#[component]
pub fn EmailChangeCompletePage() -> impl IntoView {
    view! {
        <Title text="Email Change Completed"/>
        <main class="p-4 lg:p-8">
            <div class="p-4 mx-auto max-w-md bg-white border shadow-md">
                <h1 class="mb-4 text-xl font-bold">"Email Change Completed"</h1>
                <p class="mb-4">"Your email address has been successfully changed."</p>
                <p class="mb-4">"You can now use your new email address to login."</p>
            </div>
        </main>
    }
}
