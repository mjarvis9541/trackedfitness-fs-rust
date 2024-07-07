use leptos::*;

use crate::util::validation_error::get_field_errors;

#[component]
pub fn TextInput<T>(
    #[prop(into)] name: &'static str,
    #[prop(default = name)] label: &'static str,
    #[prop(default = "text")] input_type: &'static str,
    #[prop(default = "off")] autocomplete: &'static str,
    #[prop(default = false)] disabled: bool,
    #[prop(optional, into)] value: MaybeSignal<String>,
    #[prop(optional, into)] action_value: RwSignal<Option<Result<T, ServerFnError>>>,
    #[prop(optional, into)] placeholder: MaybeSignal<String>,
) -> impl IntoView
where
    T: 'static,
{
    let field_errors = move || get_field_errors(action_value, name);
    let field_errors_view = move || {
        field_errors().map(|errors| {
            errors
                .into_iter()
                .map(|error| view! { <li>{error}</li> })
                .collect_view()
        })
    };
    let has_field_errors = move || field_errors().is_some();

    view! {
        <label class="block mb-4">
            <span class="block text-sm font-bold text-gray-700 capitalize mb-1">{label}</span>
            <input
                name=name
                value=value
                type=input_type
                disabled=disabled
                autocomplete=autocomplete
                placeholder=placeholder.get()
                class=move || {
                    if has_field_errors() {
                        "block w-full rounded border border-red-500 px-3 py-1.5 ring-2 ring-red-500 focus:outline-none"
                    } else {
                        "block w-full rounded border px-3 py-1.5 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:text-gray-800 disabled:bg-gray-300 disabled:opacity-50"
                    }
                }
            />

            <ul class="mt-1 text-red-500 font-bold">{field_errors_view}</ul>
        </label>
    }
}

#[component]
pub fn NumberInput<T>(
    #[prop(into)] name: &'static str,
    #[prop(default = name)] label: &'static str,
    #[prop(default = false)] disabled: bool,
    #[prop(default = "off")] autocomplete: &'static str,
    #[prop(default = "1")] step: &'static str,
    #[prop(optional)] max: &'static str,
    #[prop(optional)] min: &'static str,
    #[prop(optional, into)] value: MaybeSignal<String>,
    #[prop(optional, into)] action_value: RwSignal<Option<Result<T, ServerFnError>>>,
    #[prop(optional, into)] placeholder: MaybeSignal<String>,
) -> impl IntoView
where
    T: 'static,
{
    let field_errors = move || get_field_errors(action_value, name);
    let field_errors_view = move || {
        field_errors().map(|errors| {
            errors
                .into_iter()
                .map(|error| view! { <li>{error}</li> })
                .collect_view()
        })
    };
    let has_field_errors = move || field_errors().is_some();

    view! {
        <label class="block mb-4">
            <span class="block text-sm font-bold text-gray-700 capitalize mb-1">{label}</span>
            <input
                name=name
                value=value
                type="number"
                max=max
                min=min
                step=step
                disabled=disabled
                autocomplete=autocomplete
                placeholder=placeholder.get()
                class=move || {
                    if has_field_errors() {
                        "block w-full rounded border border-red-500 px-3 py-1.5 ring-2 ring-red-500 focus:outline-none "
                    } else {
                        "block w-full rounded border px-3 py-1.5 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:text-gray-800 disabled:bg-gray-300 disabled:opacity-50"
                    }
                }
            />

            <ul class="mt-1 text-red-500 font-bold">{field_errors_view}</ul>
        </label>
    }
}

// #[component]
// pub fn ValidatedInput(
//     name: &'static str,
//     #[prop(default = name.to_string(), into)] label: String,
//     #[prop(default = "1")] step: &'static str,
//     #[prop(default = "text")] input_type: &'static str,
//     #[prop(default = "off")] autocomplete: &'static str,
//     #[prop(default = false)] required: bool,
//     #[prop(optional, into)] value: MaybeSignal<String>,
//     #[prop(optional)] max: &'static str,
//     #[prop(optional)] min: &'static str,
//     #[prop(optional)] placeholder: &'static str,
//     #[prop(default = false)] disabled: bool,
//     #[prop(optional, into)] error: Signal<Option<String>>,
// ) -> impl IntoView {
//     let is_valid_input = RwSignal::new(true);

//     let field_errors = move || {
//         error.with(|err| {
//             err.as_ref().and_then(|error_message| {
//                 extract_field_errors(error_message, name).map_or_else(
//                     || {
//                         is_valid_input.update(|value| *value = true);
//                         None
//                     },
//                     |errors| {
//                         is_valid_input.update(|value| *value = false);
//                         let error_view = errors
//                         .into_iter()
//                         .map(|err| view! { <div class="mt-1 font-bold text-red-500">{err}</div> })
//                         .collect_view();
//                         Some(error_view)
//                     },
//                 )
//             })
//         })
//     };

//     let css_class = move || {
//         match is_valid_input.with(|value| *value) {
//             true => "py-1.5 px-3 w-full rounded border focus:border-blue-500 focus:ring-2 focus:ring-blue-500 focus:outline-none",
//             false => "py-1.5 px-3 w-full rounded border focus:outline-none ring-red-500 ring-2 border-red-500"
//         }
//     };
//     let handle_input = move |_ev| is_valid_input.update(|value| *value = true);
//     view! {
//         <label class="block mb-4">
//             <span class="block mb-1 font-bold">{capitalize_and_replace(&label)}</span>
//             <input
//                 type=input_type
//                 placeholder=placeholder
//                 required=required
//                 max=max
//                 min=min
//                 name=name
//                 disabled=disabled
//                 step=step
//                 value=value
//                 autocomplete=autocomplete
//                 class=css_class
//                 on:input=handle_input
//             />
//             <div class=("hidden", is_valid_input)>{field_errors}</div>
//         </label>
//     }
// }

#[component]
pub fn SetInput(
    name: &'static str,
    #[prop(default = name)] label: &'static str,
    #[prop(default = "1")] step: &'static str,
    #[prop(optional)] value: i32,
) -> impl IntoView {
    view! {
        <label class="relative w-full">
            <div class="flex absolute inset-y-0 right-0 items-center pr-2 text-gray-400 pointer-events-none select-none">
                {label}
            </div>
            <input
                type="number"
                autocomplete="off"
                name=name
                step=step
                value=value
                class="py-1.5 px-2 pr-10 w-full bg-gray-50 focus:border focus:border-blue-500 focus:ring-2 focus:ring-blue-500 focus:outline-none disabled:text-gray-800 disabled:bg-gray-300 disabled:opacity-50"
            />
        </label>
    }
}

#[component]
pub fn FilterInput(
    name: &'static str,
    value: Signal<String>,
    #[prop(default = name)] label: &'static str,
    #[prop(default = "Search")] placeholder: &'static str,
) -> impl IntoView {
    view! {
        <label class="block flex-1 min-w-40">
            <div class="mb-1 text-sm font-bold capitalize">{label}</div>
            <input
                name=name
                value=value
                type="text"
                placeholder=placeholder
                autocomplete="off"
                oninput="this.form.requestSubmit()"
                class="block py-1.5 px-3 w-full bg-white rounded border focus:border-blue-500 focus:ring-2 focus:ring-blue-500 focus:focus:outline-none"
            />
        </label>
    }
}
