use leptos::*;
use leptos_router::*;

use crate::component::calendar::Calendar;
use crate::component::dropdown::Dropdown;
use crate::component::icon::{ChevronLeft, ChevronRight, IconCalendar};
use crate::component::link::Link;
use crate::util::datetime::{Resolution, DATE_FORMAT_LONG, DATE_FORMAT_SHORT};
use crate::util::param::{
    get_current_date_url, get_date, get_next_date_url, get_previous_date_url,
};

#[component]
pub fn DateNavigation(#[prop(default = Resolution::Day)] resolution: Resolution) -> impl IntoView {
    let location = use_location();
    let pathname = move || location.pathname.get();

    let params = use_params_map();
    let date = move || get_date(&params);

    let today = move || get_current_date_url(&pathname()).unwrap();
    let next = move || get_next_date_url(&resolution, &pathname(), date()).unwrap();
    let prev = move || get_previous_date_url(&resolution, &pathname(), date()).unwrap();

    let title = move || date().format(DATE_FORMAT_LONG).to_string();
    let title_sm = move || date().format(DATE_FORMAT_SHORT).to_string();

    view! {
        <nav class="flex flex-wrap gap-2 justify-between">
            <section class="flex items-center">
                <Dropdown icon=|| view! { <IconCalendar/> }>
                    <Calendar/>
                </Dropdown>
                <h2 class="px-4 text-base font-bold whitespace-nowrap">
                    <span class="hidden lg:inline">{title}</span>
                    <span class="lg:hidden">{title_sm}</span>
                </h2>
            </section>

            <section class="flex">
                <A href=today class="flex items-center px-4 bg-gray-100 hover:bg-amber-200">
                    "Today"
                </A>
                <Link href=prev>
                    <ChevronLeft/>
                </Link>
                <Link href=next>
                    <ChevronRight/>
                </Link>
            </section>
        </nav>
    }
}
