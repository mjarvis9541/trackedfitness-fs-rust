use std::collections::HashSet;

use leptos::*;

use crate::util::text::capitalize_and_replace;

#[component]
pub fn CheckboxInput(
    name: &'static str,
    #[prop(default = name)] label: &'static str,
    #[prop(optional, into)] checked: bool,
) -> impl IntoView {
    let is_checked = RwSignal::new(checked);
    view! {
        <div>
            <label class="flex gap-4 items-center py-2 px-3 mb-4 rounded border select-none">
                <input name=name prop:value=is_checked type="hidden"/>
                <input
                    type="checkbox"
                    checked=checked
                    class="w-4 h-4 text-blue-600 rounded border-none focus:ring-2 focus:ring-blue-500"
                    on:change=move |_| is_checked.update(|value| *value = !*value)
                />
                <span class="flex text-sm font-bold">{capitalize_and_replace(label)}</span>
            </label>
        </div>
    }
}

#[component]
pub fn CheckboxListHeader(
    all_items: RwSignal<HashSet<String>>,
    checked_items: RwSignal<HashSet<String>>,
) -> impl IntoView {
    let handle_check_all = move |ev| {
        if event_target_checked(&ev) {
            checked_items.update(|v| *v = all_items.get())
        } else {
            checked_items.update(|v| v.clear())
        }
    };
    let is_all_checked =
        move || !all_items.with(HashSet::is_empty) && checked_items.with(|v| *v == all_items.get());

    view! {
        <input
            type="checkbox"
            id="header-checkbox"
            name="header-checkbox"
            prop:disabled=move || all_items.with(HashSet::is_empty)
            prop:checked=is_all_checked
            on:change=handle_check_all
        />
    }
}

#[component]
pub fn CheckboxListItem(
    id: String,
    checked_items: RwSignal<HashSet<String>>,
    #[prop(optional, into)] disabled: Signal<bool>,
) -> impl IntoView {
    let handle_check = {
        let id_clone = id.clone();
        move |ev| {
            if event_target_checked(&ev) {
                checked_items.update(|v| {
                    v.insert(id_clone.clone());
                })
            } else {
                checked_items.update(|v| {
                    v.remove(&id_clone);
                })
            }
        }
    };
    let is_checked = {
        let id_clone = id.clone();
        move || checked_items.with(|v| v.contains(&id_clone))
    };
    view! {
        <input
            type="checkbox"
            value=&id
            on:change=handle_check
            prop:checked=is_checked
            prop:disabled=disabled
        />
    }
}
