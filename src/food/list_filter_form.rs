use leptos::*;
use leptos_router::*;

use crate::brand::select::BrandFilter;
use crate::component::input::FilterInput;
use crate::component::select::FilterSelect;

#[component]
pub fn FoodListFilterForm(
    search: Signal<String>,
    brand: Signal<String>,
    serving: Signal<String>,
    order: Signal<String>,
    size: Signal<i64>,
    page: i64,
    serving_options: &'static [(&'static str, &'static str)],
    options: &'static [(&'static str, &'static str)],
) -> impl IntoView {
    view! {
        <Form method="GET" action="" class="contents">
            <FilterInput name="search" value=search/>
            <BrandFilter selected=brand/>
            <FilterSelect name="serving" value=serving options=serving_options/>
            <FilterSelect name="order" value=order options=options/>
            <input type="hidden" name="size" value=size/>
            <input type="hidden" name="page" value=page/>
        </Form>
    }
}
