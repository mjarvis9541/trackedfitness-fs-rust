use leptos::*;
use leptos_router::*;

use chrono::prelude::*;
use web_sys::TouchEvent;

use crate::util::datetime::{get_week_end, get_week_start, DateRange, Resolution};
use crate::util::misc::get_touch_coordinates;
use crate::util::param::{
    get_date, get_next_date_url, get_previous_date_url, update_date_in_url_path,
};

#[component]
pub fn WeekNavigationComponent() -> impl IntoView {
    let location = use_location();
    let path = move || location.pathname.get();

    let params = use_params_map();
    let date = move || get_date(&params);

    let now = move || Utc::now().date_naive();
    let start = move || get_week_start(date());
    let end = move || get_week_end(date());

    let range = move || {
        DateRange(start(), end())
            .map(|inner_date| {
                let href = update_date_in_url_path(&path(), &inner_date.to_string()).unwrap();
                let is_now = move || inner_date == now();
                let is_viewed = move || inner_date == date();
                let is_neither = move || inner_date != date() && inner_date != now();
                view! {
                    <a
                        href=href
                        class="flex flex-1 px-4 py-2"
                        class=("bg-blue-500", is_now)
                        class=("bg-blue-400", is_viewed)
                        class=("bg-gray-50", is_neither)
                    >
                        {inner_date.format("%a %d %b").to_string()}
                    </a>
                }
            })
            .collect_view()
    };

    let touch_start_x = RwSignal::new(0.0);
    let touch_current_x = RwSignal::new(0.0);

    let on_touch_start = move |event: TouchEvent| {
        if let Some((x, _y)) = get_touch_coordinates(&event) {
            touch_start_x.update(|v| *v = x);
            touch_current_x.update(|v| *v = x);
        }
    };

    let on_touch_move = move |event: TouchEvent| {
        if let Some((x, _y)) = get_touch_coordinates(&event) {
            touch_current_x.update(|v| *v = x);
        }
    };

    let on_touch_end = move |_event: TouchEvent| {
        let start_x = touch_start_x.get();
        let end_x = touch_current_x.get();
        let threshold = 50.0; // Minimum swipe distance in pixels

        if (start_x - end_x).abs() > threshold {
            leptos::logging::log!("threshold");
            let navigate = use_navigate();
            if end_x < start_x {
                // Swipe left
                let href = get_next_date_url(&Resolution::Week, &path(), date()).unwrap();
                navigate(&href, Default::default())
            } else {
                // Swipe right
                let href = get_previous_date_url(&Resolution::Week, &path(), date()).unwrap();
                navigate(&href, Default::default())
            }
        }
    };
    view! {
        <nav
            class="flex gap-2 overflow-x-auto"
            on:touchstart=on_touch_start
            on:touchmove=on_touch_move
            on:touchend=on_touch_end
        >
            {range}
        </nav>

        <div>

            <div>"date: " {move || format!("{:?}", date())}</div>
            <div>"touch x start: " {touch_start_x}</div>
            <div>"touch x current: " {touch_current_x}</div>
        </div>
    }
}
