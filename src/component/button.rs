use leptos::*;

#[derive(Debug)]
pub enum ButtonVariant {
    Primary,
    Danger,
}

impl ButtonVariant {
    pub fn into_css(&self) -> &'static str {
        match self {
            ButtonVariant::Primary => "flex gap-2 whitespace-nowrap p-2 bg-gray-100 hover:bg-amber-200 disabled:opacity-50 disabled:pointer-events-none",
            ButtonVariant::Danger => "flex gap-2 whitespace-nowrap p-2 bg-gray-200 hover:bg-red-500 hover:text-white disabled:opacity-50 disabled:pointer-events-none",
        }
    }
}

#[component]
pub fn Button(
    #[prop(optional)] label: &'static str,
    #[prop(optional, into)] loading: MaybeSignal<bool>,
    #[prop(optional, into)] disabled: MaybeSignal<bool>,
    #[prop(optional)] children: Option<Children>,
    #[prop(default = ButtonVariant::Primary)] variant: ButtonVariant,
) -> impl IntoView {
    let is_disabled = move || disabled.with(|value| *value) | loading.with(|value| *value);
    view! {
        <button class=variant.into_css() prop:disabled=is_disabled>
            {children.map(|children| children())}
            {label}
        </button>
    }
}

#[component]
pub fn SubmitButton(
    #[prop(default = "Submit")] label: &'static str,
    #[prop(optional, into)] loading: MaybeSignal<bool>,
    #[prop(optional, into)] disabled: MaybeSignal<bool>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let button_text = move || loading.with(|v| if *v { "Loading..." } else { label });
    let is_disabled = move || disabled.with(|value| *value) | loading.with(|value| *value);
    view! {
        <button
            {..attrs}
            class="block py-2 px-3 w-full font-semibold whitespace-nowrap rounded duration-300 hover:text-white focus:ring-2 focus:ring-blue-500 disabled:opacity-50 disabled:pointer-events-none text-zinc-100 bg-zinc-900 hover:bg-zinc-700 focus:focus:outline-none"
            prop:disabled=is_disabled
        >
            {button_text}
        </button>
    }
}
