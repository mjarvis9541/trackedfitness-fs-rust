use leptos::*;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::util::text::capitalize_and_replace;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SelectUuidName {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SelectSlugName {
    pub slug: String,
    pub name: String,
}

#[component]
pub fn FilterSelect(
    name: &'static str,
    #[prop(default = name)] label: &'static str,
    value: Signal<String>,
    options: Vec<(&'static str, &'static str)>,
) -> impl IntoView {
    let options_view = move || {
        options
            .clone()
            .into_iter()
            .map(|(option_value, label)| {
                let selected_value = value.get();
                view! {
                    <option
                        value=option_value
                        prop:selected=move || *option_value == selected_value
                    >
                        {label}
                    </option>
                }
            })
            .collect_view()
    };
    view! {
        <label class="block flex-1 min-w-40" aria-labelledby=format!("{}-label", name)>
            <span id=format!("{}-label", name) class="block mb-1 font-bold capitalize">
                {label}
            </span>
            <select
                name=name
                onchange="this.form.requestSubmit()"
                class="flex py-1.5 px-3 w-full bg-white rounded border focus:border-blue-500 focus:ring-2 focus:ring-blue-500 focus:outline-none disabled:bg-gray-500 disabled:opacity-50 h-[34px] placeholder:text-gray-400 disabled:placeholder:text-gray-500"
                aria-labelledby=format!("{}-label", name)
            >
                {options_view}
            </select>
        </label>
    }
}

#[component]
pub fn FieldSelect(
    name: &'static str,
    #[prop(default = name)] label: &'static str,
    #[prop(optional, into)] value: MaybeSignal<String>,
    options: Vec<(&'static str, &'static str)>,
) -> impl IntoView {
    let options_view = move || {
        options
            .clone()
            .into_iter()
            .map(|(option_value, label)| {
                let selected_value = value.get();
                view! {
                    <option
                        value=option_value
                        prop:selected=move || *option_value == selected_value
                    >
                        {label}
                    </option>
                }
            })
            .collect_view()
    };
    view! {
        <label class="block mb-4">
            <span class="block mb-1 text-sm font-bold">{capitalize_and_replace(label)}</span>
            <select
                name=name
                class="block py-2.5 px-3 w-full bg-white rounded border focus:border-blue-500 focus:ring-2 focus:ring-blue-500 focus:outline-none"
            >
                {options_view}
            </select>
        </label>
    }
}

#[component]
pub fn SelectOption(
    value: &'static str,
    label: &'static str,
    selected: Signal<String>,
) -> impl IntoView {
    view! {
        <option value=value selected=move || selected() == value>
            {label}
        </option>
    }
}
