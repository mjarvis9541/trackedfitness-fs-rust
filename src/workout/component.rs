use leptos::*;
use leptos_meta::Title;
use leptos_router::*;

use crate::component::icon::IconFilePlus;
use crate::component::link::Link;
use crate::util::datetime::DATE_FORMAT_LONG;
use crate::util::param::{generate_create_workout_url, get_date};

#[component]
pub fn WorkoutDayHeader(title: &'static str) -> impl IntoView {
    let params = use_params_map();
    let date = move || get_date(&params);

    let workout_date_title = move || date().format(DATE_FORMAT_LONG).to_string();
    let create_workout_url = move || generate_create_workout_url(&params);

    view! {
        <Title text=title/>
        <header class="flex gap-2 items-start p-2 mb-2 bg-gray-200/75">

            <section class="flex-1">
                <h1 class="text-base font-bold">"Workouts"</h1>
                <p class="mb-2 text-xs text-gray-500">{workout_date_title}</p>
            </section>

            <section class="flex gap-2">
                <Link href=create_workout_url text="New Workout">
                    <IconFilePlus/>
                </Link>
            </section>

        </header>
    }
}
