use std::collections::HashSet;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::component::checkbox::{CheckboxListHeader, CheckboxListItem};
use crate::component::icon::{IconEditA, IconTrash};
use crate::component::input::FilterInput;
use crate::component::link::Link;
use crate::component::select::FilterSelect;

#[component]
pub fn DetailPageTemplate(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <Title text=title/>
        <main class="mx-auto mt-8 p-6 bg-white shadow-md rounded-lg max-w-lg">
            <h1 class="mb-2 text-2xl font-bold">{title}</h1>
            {children()}
        </main>
    }
}

#[component]
pub fn CreateButton(
    #[prop(into)] text: MaybeSignal<String>,
    #[prop(into)] create_href: MaybeSignal<String>,
) -> impl IntoView {
    view! {
        <div class="flex gap-2 justify-end pt-4">
            <a class="block py-1.5 px-3 bg-gray-200 rounded hover:bg-gray-300" href=create_href>
                {text}
            </a>
        </div>
    }
}

#[component]
pub fn UpdateDeleteButtonRow() -> impl IntoView {
    view! {
        <section class="flex gap-2 justify-end mt-4">
            <Link text="Edit" href="update">
                <IconEditA/>
            </Link>
            <Link text="Delete" href="delete">
                <IconTrash/>
            </Link>
        </section>
    }
}

#[component]
pub fn AdminCreate() -> impl IntoView {
    view! {
        <section class="flex gap-2 justify-end mt-4">
            <Link text="Edit" href="update">
                <IconEditA/>
            </Link>
            <Link text="Delete" href="delete">
                <IconTrash/>
            </Link>
        </section>
    }
}

#[component]
pub fn ViewEditDeleteButtonRow(
    #[prop(into)] detail: MaybeSignal<String>,
    #[prop(into)] update: MaybeSignal<String>,
    #[prop(into)] delete: MaybeSignal<String>,
) -> impl IntoView {
    view! {
        <div class="flex gap-2 justify-end pt-4">
            <a class="block py-1.5 px-3 bg-gray-200 rounded hover:bg-gray-300" href=detail>
                "View"
            </a>
            <a class="block py-1.5 px-3 bg-gray-200 rounded hover:bg-gray-300" href=update>
                "Edit"
            </a>
            <a class="block py-1.5 px-3 bg-gray-200 rounded hover:bg-gray-300" href=delete>
                "Delete"
            </a>
        </div>
    }
}

#[component]
pub fn Backdrop(show_menu: RwSignal<bool>) -> impl IntoView {
    let close_menu = move |_| show_menu.update(|value| *value = false);
    let is_shown = move || show_menu.with(|value| *value);
    let is_hidden = move || show_menu.with(|value| !*value);
    view! {
        <div
            on:click=close_menu
            class="fixed inset-0 z-10 transition duration-300"
            class=("pointer-events-none", is_hidden)
            class=("pointer-events-auto", is_shown)
            class=("bg-black/50", is_shown)
        ></div>
    }
}

#[component]
pub fn LoadingSpinner() -> impl IntoView {
    view! {
        <svg
            class="w-8 h-8 text-gray-200 animate-spin fill-blue-600"
            viewBox="0 0 100 101"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z"
                fill="currentColor"
            ></path>
            <path
                d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z"
                fill="currentFill"
            ></path>
        </svg>
    }
}

#[component]
pub fn Loading() -> impl IntoView {
    view! { <span>"Loading..."</span> }
}

#[component]
pub fn OptionLoading() -> impl IntoView {
    view! { <option>"Loading..."</option> }
}

#[component]
pub fn OptionError() -> impl IntoView {
    view! { <OptionError/> }
}

#[component]
pub fn LoadingComponent() -> impl IntoView {
    view! { <div>"Loading..."</div> }
}

#[component]
pub fn ListLoadingComponent() -> impl IntoView {
    view! { <div class="flex col-span-full items-center px-4 h-11 text-gray-500">"Loading..."</div> }
}

#[component]
pub fn ListNotFoundComponent() -> impl IntoView {
    view! { <div class="flex col-span-full items-center px-4 h-11 text-gray-500">"No results"</div> }
}

#[component]
pub fn FoodListItemMacroHeader() -> impl IntoView {
    view! {
        <div class="block px-2 pt-1 text-xs text-right md:hidden group-hover:bg-gray-200 group-odd:bg-gray-50">
            "Calories"
        </div>
        <div class="block px-2 pt-1 text-xs text-right md:hidden group-hover:bg-gray-200 group-odd:bg-gray-50">
            "Protein"
        </div>
        <div class="block px-2 pt-1 text-xs text-right md:hidden group-hover:bg-gray-200 group-odd:bg-gray-50">
            "Carbs"
        </div>
        <div class="block px-2 pt-1 text-xs text-right md:hidden group-hover:bg-gray-200 group-odd:bg-gray-50">
            "Fat"
        </div>
    }
}

#[component]
pub fn Skeleton(#[prop(into)] row_count: MaybeSignal<usize>) -> impl IntoView {
    let row_count = row_count.get();
    move || {
        (0..row_count)
            .map(|_| {
                view! { <div class="col-span-full mb-1 h-10 bg-gray-200 animate-pulse"></div> }
            })
            .collect_view()
    }
}

#[component]
pub fn DeletePageWrapper(
    #[prop(optional, into)] title: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    let ref_title = &title.get();
    let page_title = format!("Delete {}", ref_title);

    view! {
        <Title text=page_title.clone()/>
        <main class="p-4 m-4 max-w-md bg-white border">
            <h1 class="mb-4 text-xl font-bold">{page_title}</h1>
            <p class="mb-4">"Are you sure you wish to delete this " {title} "?"</p>
            <p class="mb-4">"Ths action cannot be undone."</p>
            {children()}
        </main>
    }
}

#[component]
pub fn ListPageHeaderWithCreate(
    #[prop(optional, into)] title: MaybeSignal<String>,
    #[prop(optional, into)] subtitle: MaybeSignal<String>,
    #[prop(optional, into)] create_href: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <header class="flex flex-wrap justify-between items-start mb-2">
            <div>
                <h1 class="text-xl font-bold">{title}</h1>
                <p class="text-gray-400">{subtitle}</p>
                <p class="text-gray-400">"Results: " {children()}</p>
            </div>
            <div>
                <Link href=create_href.get() text="New">
                    <IconEditA/>
                </Link>
            </div>
        </header>
    }
}

#[component]
pub fn AutoListHeader(
    children: Children,
    all_items: RwSignal<HashSet<String>>,
    checked_items: RwSignal<HashSet<String>>,
    #[prop(optional)] align_right: bool,
) -> impl IntoView {
    let row_class = if align_right {
        "hidden p-2 font-bold whitespace-nowrap border-b md:flex items-center justify-end"
    } else {
        "hidden p-2 font-bold whitespace-nowrap border-b md:flex items-center justify-end"
    };
    let children = children()
        .nodes
        .into_iter()
        .map(|child| view! { <div class=row_class>{child}</div> })
        .collect_view();
    view! {
        <div class=row_class>
            <CheckboxListHeader all_items checked_items/>
        </div>
        {children}
    }
}

#[component]
pub fn AutoListItem(
    #[prop(into)] id: MaybeSignal<String>,
    checked_items: RwSignal<HashSet<String>>,
    children: Children,
) -> impl IntoView {
    let children = children()
        .nodes
        .into_iter()
        .map(|child| view! {
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50 truncate">
                {child}
            </div>
        })
        .collect_view();
    view! {
        <div class="contents group">
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50 truncate">
                <CheckboxListItem id=id.get() checked_items/>
            </div>
            {children}
        </div>
    }
}

#[component]
pub fn AddFoodListHeader(
    #[prop(optional, into)] title: MaybeSignal<String>,
    #[prop(optional, into)] subtitle: MaybeSignal<String>,
) -> impl IntoView {
    view! {
        <div class="hidden col-span-3 p-2 font-bold border-b lg:block">{title}</div>
        <div class="hidden p-2 font-bold text-right border-b lg:block">{subtitle}</div>
        <div class="hidden p-2 font-bold text-right border-b lg:block">"Calories"</div>
        <div class="hidden p-2 font-bold text-right border-b lg:block">"Protein"</div>
        <div class="hidden p-2 font-bold text-right border-b lg:block">"Carbs"</div>
        <div class="hidden p-2 font-bold text-right border-b lg:block">"Fat"</div>
        <div class="hidden p-2 font-bold text-right border-b lg:block">"Sat Fat."</div>
        <div class="hidden p-2 font-bold text-right border-b lg:block">"Sugars"</div>
        <div class="hidden p-2 font-bold text-right border-b lg:block">"Fibre"</div>
        <div class="hidden p-2 font-bold text-right border-b lg:block">"Salt"</div>
        <div class="hidden p-2 font-bold text-right border-b lg:block"></div>
    }
}

#[component]
pub fn SearchForm(
    search: Signal<String>,
    order: Signal<String>,
    size: Signal<i64>,
    page: i64,
    options: &'static [(&'static str, &'static str)],
) -> impl IntoView {
    view! {
        <Form method="GET" action="" class="contents">
            <FilterInput name="search" value=search/>
            <FilterSelect name="order" value=order options/>
            <input type="hidden" name="size" value=size/>
            <input type="hidden" name="page" value=page/>
        </Form>
    }
}

#[component]
pub fn ErrorComponent(errors: RwSignal<Errors>) -> impl IntoView {
    errors
        .get_untracked()
        .into_iter()
        .map(|(_, e)| view! { <div class="font-bold text-red-500">{e.to_string()}</div> })
        .collect_view()
}

#[component]
pub fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    if let Some(res) = use_context::<leptos_axum::ResponseOptions>() {
        res.set_status(http::StatusCode::NOT_FOUND)
    }
    view! {
        <Title text="Not Found"/>
        <main class="p-4">
            <div class="p-4 bg-white border">
                <h1 class="mb-4 text-base font-bold">"Page Not Found"</h1>
                <p class="mb-4">"Page not found"</p>
                <p class="mb-4">
                    <a href="/" class="text-blue-500 hover:underline">
                        "Home"
                    </a>
                </p>
            </div>
        </main>
    }
}