use leptos::*;

#[component]
pub fn ToastSuccess(
    #[prop(into, optional)] update_show: Signal<bool>,
    #[prop(default = "Success!")] title: &'static str,
    #[prop(default = "Your action was successful.")] message: &'static str,
) -> impl IntoView {
    let is_shown = RwSignal::new(false);
    let hide_element = move |_| is_shown.update(|value| *value = false);

    view! {
        <Show when=is_shown>
            <div class="fixed inset-0 z-10 duration-300 bg-black/50" on:click=hide_element>
                <div class="relative top-1/4 z-20 p-4 mx-auto mt-4 max-w-sm text-white bg-green-500 rounded-md shadow-md">

                    <div class="flex items-center">

                        <div class="flex flex-1 items-center">
                            <svg
                                class="mr-2 w-6 h-6"
                                fill="none"
                                stroke="currentColor"
                                viewBox="0 0 24 24"
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M5 13l4 4L19 7"
                                ></path>
                            </svg>
                            <span class="font-bold">{title}</span>
                        </div>

                        <div>
                            <button on:click=hide_element>"Close"</button>
                        </div>
                    </div>
                    <p class="mt-2">{message}</p>
                </div>

            </div>
        </Show>
        {move || is_shown.update(|value| *value = update_show.get())}
    }
}
