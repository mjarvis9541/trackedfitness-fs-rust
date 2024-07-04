use leptos::*;
use leptos_router::*;

use chrono::Utc;

use crate::component::icon::{ChevronLeft, ChevronRight};
use crate::util::datetime::{
    get_month_end_comprehensive, get_month_start_comprehensive, DateRange, Resolution,
};
use crate::util::param::{
    get_date, get_next_date_url, get_previous_date_url, update_date_in_url_path,
};

#[component]
pub fn Calendar() -> impl IntoView {
    let location = use_location();
    let path = move || location.pathname.get();

    let params = use_params_map();
    let date = move || get_date(&params);

    let now = move || Utc::now().date_naive();
    let start = move || get_month_start_comprehensive(date());
    let end = move || get_month_end_comprehensive(date());

    let date_range_view = move || {
        DateRange(start(), end())
            .map(|inner_date| {
                let href = update_date_in_url_path(&path(), &inner_date.to_string()).unwrap();

                let is_now = move || inner_date == now();
                let is_viewed = move || inner_date == date();
                let is_neither = move || inner_date != date() && inner_date != now();

                view! {
                    <a
                        href=href
                        class="flex flex-1 items-center justify-center p-2 hover:bg-gray-300"
                        class=("bg-blue-500", is_now)
                        class=("text-white", is_now)
                        class=("bg-blue-400", is_viewed)
                        class=("text-white", is_viewed)
                        class=("bg-gray-50", is_neither)
                    >
                        {inner_date.format("%-d").to_string()}
                    </a>
                }
            })
            .collect_view()
    };

    let next = move || get_next_date_url(&Resolution::Month, &path(), date()).unwrap();
    let prev = move || get_previous_date_url(&Resolution::Month, &path(), date()).unwrap();

    let title = move || date().format("%B %Y").to_string();

    view! {
        <div class="grid grid-cols-7 text-xs bg-white select-none">
            <div class="flex flex-wrap col-span-full justify-between">
                <div class="flex flex-1 items-center p-2 text-sm font-bold whitespace-nowrap">
                    {title}
                </div>
                <div class="flex">
                    <a class="block p-2 duration-300 hover:bg-gray-300" href=prev>
                        <ChevronLeft/>
                    </a>
                    <a class="block p-2 duration-300 hover:bg-gray-300" href=next>
                        <ChevronRight/>
                    </a>
                </div>
            </div>
            <div class="flex justify-center items-center p-2 w-8">"M"</div>
            <div class="flex justify-center items-center p-2 w-8">"T"</div>
            <div class="flex justify-center items-center p-2 w-8">"W"</div>
            <div class="flex justify-center items-center p-2 w-8">"T"</div>
            <div class="flex justify-center items-center p-2 w-8">"F"</div>
            <div class="flex justify-center items-center p-2 w-8">"S"</div>
            <div class="flex justify-center items-center p-2 w-8">"S"</div>
            {date_range_view}
        </div>
    }
}
