use leptos::server_fn::codec::GetUrl;
use leptos::*;

use uuid::Uuid;

use crate::component::select::{SelectSlugName, SelectUuidName};
use crate::component::template::{OptionError, OptionLoading};

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user, muscle_group::model::MuscleGroupQuery, setup::get_pool,
};

#[server(endpoint = "muscle-group-select-slug", input = GetUrl)]
pub async fn get_muscle_group_filter() -> Result<Vec<SelectSlugName>, ServerFnError> {
    let _user = get_request_user()?;
    let pool = get_pool()?;
    Ok(MuscleGroupQuery::option_list_slug(&pool).await?)
}

#[server(endpoint = "muscle-group-select-id", input = GetUrl)]
pub async fn get_muscle_group_form_select() -> Result<Vec<SelectUuidName>, ServerFnError> {
    let _user = get_request_user()?;
    let pool = get_pool()?;
    Ok(MuscleGroupQuery::option_list_id(&pool).await?)
}

#[component]
pub fn MuscleGroupFormSelect(#[prop(optional)] selected: Uuid) -> impl IntoView {
    let resource = Resource::once(get_muscle_group_form_select);

    let response = move || {
        resource.and_then(|data| {
            data.clone()
                .into_iter()
                .map(|option| {
                    view! {
                        <option
                            prop:value=option.id.to_string()
                            prop:selected=move || option.id == selected
                        >
                            {option.name}
                        </option>
                    }
                })
                .collect_view()
        })
    };
    view! {
        <label class="block mb-4">
            <span class="block mb-1 text-sm font-bold">"Muscle Group"</span>
            <select
                name="muscle_group_id"
                class="block py-2 px-3 w-full bg-white rounded border focus:border-blue-500 focus:ring-2 focus:ring-blue-500 focus:outline-none"
            >
                <Transition fallback=OptionLoading>
                    <ErrorBoundary fallback=|_| {
                        view! { <OptionError/> }
                    }>{response}</ErrorBoundary>
                </Transition>
            </select>
        </label>
    }
}

#[component]
pub fn MuscleGroupFilter(#[prop(into)] selected: Signal<String>) -> impl IntoView {
    let resource = expect_context::<Resource<(), Result<Vec<SelectSlugName>, ServerFnError>>>();

    let response = move || {
        resource.and_then(|data| {
            data.iter()
                .map(|option| {
                    let slug = &option.slug;
                    let name = &option.name;
                    let is_selected = selected.with(|s| s == slug);
                    view! {
                        <option value=slug selected=is_selected>
                            {name}
                        </option>
                    }
                })
                .collect_view()
        })
    };
    view! {
        <label class="block flex-1 min-w-40">
            <span class="block mb-1 font-bold">"Muscle Group"</span>
            <select
                name="muscle_group"
                onchange="this.form.requestSubmit()"
                class="flex py-1.5 px-3 w-full bg-white rounded border focus:border-blue-500 focus:ring-2 focus:ring-blue-500 focus:outline-none disabled:bg-gray-500 disabled:opacity-50 h-[34px] placeholder:text-gray-400 disabled:placeholder:text-gray-500"
            >
                <Transition fallback=OptionLoading>
                    <ErrorBoundary fallback=|_| {
                        view! { <OptionError/> }
                    }>
                        <option value="">"All"</option>
                        {response}
                    </ErrorBoundary>
                </Transition>
            </select>
        </label>
    }
}
