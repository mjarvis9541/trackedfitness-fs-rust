use leptos::*;

use crate::component::icon::MoreHorizontal;
use crate::component::template::Backdrop;

#[component]
pub fn Dropdown(
    #[prop(optional, into)] icon: Option<ViewFn>,
    #[prop(optional)] position_right: bool,
    children: Children,
) -> impl IntoView {
    let show_menu = RwSignal::new(false);
    let toggle_menu = move |_| show_menu.update(|value| *value = !*value);
    let is_shown = move || show_menu.with(|value| *value);
    let is_hidden = move || show_menu.with(|value| !*value);
    let icon_view = move || icon.map_or_else(|| view! { <MoreHorizontal/> }, |icon| icon.run());
    view! {
        <Backdrop show_menu/>
        <div class="relative">
            <button class="p-2 bg-gray-100 hover:bg-amber-200" on:click=toggle_menu>
                {icon_view()}
            </button>
            <div
                class="absolute z-10 grid w-56 transition-grid-template-rows duration-300"
                class=("left-0", !position_right)
                class=("right-0", position_right)
                class=("grid-rows-row-hide", is_hidden)
                class=("grid-rows-row-show", is_shown)
            >
                <div class="overflow-hidden bg-white">
                    <div class="p-2 border">{children()}</div>
                </div>
            </div>
        </div>
    }
}
