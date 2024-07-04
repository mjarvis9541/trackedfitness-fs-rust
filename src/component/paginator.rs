use leptos::*;
use leptos_router::*;

use crate::component::icon::{ChevronLeft, ChevronRight, ChevronsLeft, ChevronsRight};
use crate::component::link::{Link, LinkVariant};
use crate::component::select::SelectOption;
use crate::util::param::{extract_page, extract_size};

fn generate_page_url(query_string: &str, current_page: i64, target_page: i64) -> String {
    if query_string.is_empty() {
        if current_page == target_page {
            String::new()
        } else {
            format!("?page={}", target_page)
        }
    } else if query_string.contains("page") {
        query_string.replace(
            &format!("page={}", current_page),
            &format!("page={}", target_page),
        )
    } else {
        format!("{}&page={}", query_string, target_page)
    }
}

fn generate_next_page_url(query_string: &str, current_page: i64, total_pages: i64) -> String {
    if total_pages == 1 || current_page == total_pages {
        query_string.to_string()
    } else {
        generate_page_url(query_string, current_page, current_page + 1)
    }
}

fn generate_previous_page_url(query_string: &str, current_page: i64) -> String {
    if current_page <= 1 {
        query_string.to_string()
    } else {
        generate_page_url(query_string, current_page, current_page - 1)
    }
}

#[component]
pub fn Paginator(#[prop(into)] count: Signal<Option<i64>>) -> impl IntoView {
    let query = use_query_map();
    let size = move || extract_size(&query);
    let page = move || extract_page(&query);

    let total_pages = move || {
        let count_value = count().unwrap_or(0);
        let size_value = size();
        if count_value == 0 || size_value == 0 {
            1
        } else {
            (count_value - 1) / size_value + 1
        }
    };

    let query_string = move || query.with(|q| q.to_query_string());

    let first_page = move || generate_page_url(&query_string(), page(), 1);
    let last_page = move || generate_page_url(&query_string(), page(), total_pages());
    let next_page = move || generate_next_page_url(&query_string(), page(), total_pages());
    let previous_page = move || generate_previous_page_url(&query_string(), page());

    view! {
        <div class="flex flex-wrap justify-end select-none">

            <PageSizeFilter selected=Signal::derive(move || size().to_string())/>

            <div class="flex">
                <div class="flex justify-center items-center px-2 whitespace-nowrap min-w-32">
                    "Page " {page} " of " {total_pages}
                </div>

                <Link variant=LinkVariant::Page href=first_page>
                    <ChevronsLeft/>
                </Link>
                <Link variant=LinkVariant::Page href=previous_page>
                    <ChevronLeft/>
                </Link>
                <Link variant=LinkVariant::Page href=next_page>
                    <ChevronRight/>
                </Link>
                <Link variant=LinkVariant::Page href=last_page>
                    <ChevronsRight/>
                </Link>
            </div>

        </div>
    }
}

#[component]
pub fn PageSizeFilter(selected: Signal<String>) -> impl IntoView {
    view! {
        <label class="flex items-center">
            <div class="px-4">"Show"</div>
            <select
                name="size"
                onchange="this.form.requestSubmit()"
                class="flex py-1.5 px-3 w-full bg-white rounded border focus:border-blue-500 focus:ring-2 focus:ring-blue-500 focus:outline-none disabled:bg-gray-500 disabled:opacity-50 h-[34px] placeholder:text-gray-400 disabled:placeholder:text-gray-500"
            >
                <SelectOption selected value="10" label="10"/>
                <SelectOption selected value="25" label="25"/>
                <SelectOption selected value="50" label="50"/>
                <SelectOption selected value="75" label="75"/>
                <SelectOption selected value="100" label="100"/>
            </select>
        </label>
    }
}
