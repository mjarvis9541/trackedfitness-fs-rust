use leptos::*;

use crate::component::button::Button;
use crate::component::icon::IconClose;

#[component]
pub fn Modal(
    #[prop(optional, into)] title: &'static str,
    #[prop(optional, into)] text: &'static str,
    show: RwSignal<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let close_menu = move |_| show.update(|value| *value = false);

    view! {
        <Show when=show>
            <div class="fixed inset-0 z-10 transition-opacity duration-300" on:click=close_menu>
                <div class="relative top-40 z-20 p-2 m-auto max-w-lg bg-white rounded border drop-shadow-md">

                    <header class="flex justify-between items-center p-2 border-b">
                        <h2 class="text-base font-bold">{title}</h2>
                        <Button on:click=close_menu>
                            <IconClose/>
                        </Button>
                    </header>

                    <section class="p-2">
                        <p class="py-4">{text}</p>
                        {children()}
                    </section>

                </div>
            </div>
        </Show>
    }
}

#[component]
pub fn ErrorModal(
    #[prop(optional, into)] title: &'static str,
    #[prop(optional, into)] message: String,
    show: RwSignal<bool>,
) -> impl IntoView {
    let close_menu = move |_| show.update(|value| *value = false);

    view! {
        <Show when=show>
            <div class="fixed inset-0 z-10 duration-300 bg-black/50" on:click=close_menu>
                <div class="relative top-40 z-20 p-2 m-auto max-w-lg bg-white rounded border drop-shadow-md">

                    <header class="flex justify-between items-center">
                        <h1 class="text-xl font-bold">{title}</h1>
                        <Button on:click=close_menu>
                            <IconClose/>
                        </Button>
                    </header>

                    <section class="p-2">
                        <p class="py-4 font-bold text-red-500 capitalize">{&message}</p>
                    </section>

                </div>
            </div>
        </Show>
    }
}
