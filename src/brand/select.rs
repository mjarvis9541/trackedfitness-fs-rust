use leptos::*;

use uuid::Uuid;

use crate::component::select::{SelectSlugName, SelectUuidName};
use crate::component::template::{OptionError, OptionLoading};

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, brand::model::BrandQuery, setup::get_pool};

#[server]
pub async fn get_brand_filter() -> Result<Vec<SelectSlugName>, ServerFnError> {
    get_request_user()?;
    let pool = get_pool()?;

    let query = BrandQuery::option_list_slug(&pool).await?;
    Ok(query)
}

#[server]
pub async fn get_brand_select() -> Result<Vec<SelectUuidName>, ServerFnError> {
    get_request_user()?;
    let pool = get_pool()?;
    let query = BrandQuery::option_list_id(&pool).await?;
    Ok(query)
}

#[component]
pub fn BrandSelect(#[prop(optional)] selected: Uuid) -> impl IntoView {
    let resource = expect_context::<
        Resource<(), Result<Vec<crate::component::select::SelectUuidName>, ServerFnError>>,
    >();
    let response = move || {
        resource.and_then(|data| {
            data.clone()
                .into_iter()
                .map(|option| {
                    view! {
                        <option value=option.id.to_string() selected=move || option.id == selected>
                            {option.name}
                        </option>
                    }
                })
                .collect_view()
        })
    };
    view! {
        <label class="block mb-4">
            <span class="block mb-1 text-sm font-bold">"Brand"</span>
            <select
                name="brand_id"
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
pub fn BrandFilter(selected: Signal<String>) -> impl IntoView {
    let resource = Resource::once(get_brand_filter);

    let response = move || {
        resource.and_then(|data| {
            data.clone()
                .into_iter()
                .map(|option| {
                    view! {
                        <option value=&option.slug selected=move || option.slug == selected()>
                            {option.name}
                        </option>
                    }
                })
                .collect_view()
        })
    };
    view! {
        <label class="block flex-1 min-w-40">
            <span class="block mb-1 text-sm font-bold">"Brand"</span>
            <select
                name="brand"
                onchange="this.form.requestSubmit()"
                class="flex py-1.5 px-3 w-full bg-white rounded border focus:border-blue-500 focus:ring-2 focus:ring-blue-500 focus:outline-none disabled:bg-gray-500 disabled:opacity-50 h-[34px] placeholder:text-gray-400 disabled:placeholder:text-gray-500"
            >
                <option value="">"All"</option>
                <Transition fallback=OptionLoading>
                    <ErrorBoundary fallback=|_| {
                        view! { <OptionError/> }
                    }>{response}</ErrorBoundary>
                </Transition>
            </select>
        </label>
    }
}
