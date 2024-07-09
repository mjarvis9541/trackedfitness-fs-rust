use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use chrono::prelude::*;

use super::model::MonthSummary;
use crate::component::badge::{BadgeDiet, BadgeProgress, Badgei64};
use crate::component::date_navigation::DateNavigation;
use crate::component::template::{ErrorComponent, LoadingComponent};
use crate::util::datetime::Resolution;
use crate::util::param::{get_date, get_username};

#[cfg(feature = "ssr")]
use crate::{auth::model::User, auth::service::get_request_user, setup::get_pool};

#[server]
pub async fn get_month_summary(
    username: String,
    date: NaiveDate,
) -> Result<Vec<MonthSummary>, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    User::check_view_permission(&pool, &user, &username).await?;
    let query = MonthSummary::get(&pool, &username, date).await?;
    Ok(query)
}

#[component]
pub fn UserSummaryMonthPage() -> impl IntoView {
    let params = use_params_map();
    let username = move || get_username(&params);
    let date = move || get_date(&params);

    let resource = Resource::new(
        move || (username(), date()),
        |(username, date)| get_month_summary(username, date),
    );
    let response = move || {
        resource.and_then(|data| {
            data.iter()
                .map(|data| match data.date.weekday() {
                    Weekday::Sun => view! {
                        <MonthListItem data=data.clone() param_date=Signal::derive(date)/>
                        <MonthListItemTotal data=data.clone()/>
                    }
                    .into_view(),
                    _ => {
                        view! { <MonthListItem data=data.clone() param_date=Signal::derive(date)/> }
                    }
                })
                .collect_view()
        })
    };

    // let polyline_points = RwSignal::new(String::new());
    // let y_min_value = 70.0;
    // let y_max_value = 100.0;
    // let x_axis_data = move || generate_month_range(date());
    // let chart_data = move || {
    //     month_resource.and_then(|data| {
    //         polyline_points.update(|v| v.clear());

    //         let chart_left = 50.0;
    //         let chart_right = 1250.0;
    //         let chart_top = 50.0;
    //         let chart_bottom = 350.0;

    //         let first = get_month_start(date());
    //         let last = get_month_end(date());

    //         let x_min_value = first.day() as f64;
    //         let x_max_value = last.day() as f64;

    //         data.iter()
    //             .filter(|x| x.date >= first && x.date <= last)
    //             .map(|inner| {
    //                 let data_point = inner.weight.map(|weight| {
    //                     let date_f64 = inner.date.day() as f64;
    //                     let weight_f64 = weight.to_f64().unwrap_or_default();

    //                     let x = map_value_to_range(
    //                         date_f64,
    //                         x_min_value,
    //                         x_max_value,
    //                         chart_left,
    //                         chart_right,
    //                     );
    //                     let y = map_value_to_range(
    //                         weight_f64,
    //                         y_min_value,
    //                         y_max_value,
    //                         chart_bottom,
    //                         chart_top,
    //                     );
    //                     // leptos::logging::log!("x: {}, y: {}", x, y);
    //                     polyline_points.update(|v| *v = format!("{}{},{} ", v, x, y));
    //                     view! { <Circle cx=x cy=y r=5.0/> }
    //                 });

    //                 data_point
    //             })
    //             .collect_view()
    //     })
    // };

    view! {
        <Title text="Month"/>
        <main class="p-4 m-4 bg-white border">
            <div class="pb-4">
                <DateNavigation resolution=Resolution::Month/>
            </div>

            // <h2 class="font-bold mb-2 text-xl">"Weight"</h2>
            // <div class="flex overflow-x-auto snap-x snap-mandatory">
            // <div class="mx-auto min-w-[1300px]">
            // <Transition>
            // <LineChart
            // y_min=y_min_value
            // y_max=y_max_value
            // x_axis_data=Signal::derive(x_axis_data)
            // >
            // <Polyline
            // points=polyline_points
            // attr:fill="none"
            // attr:stroke="3"
            // attr:stroke-dasharray="2000"
            // attr:stroke-dashoffset="2000"
            // attr:class="stroke-red-500 stroke-3 animate-line"
            // />
            // {chart_data}
            // </LineChart>
            // </Transition>
            // </div>
            // </div>
            <div class="grid grid-cols-1 gap-2 md:grid-cols-4 lg:grid-cols-8">
                <div class="hidden justify-center p-2 pb-0 font-bold lg:flex">"Mon"</div>
                <div class="hidden justify-center p-2 pb-0 font-bold lg:flex">"Tue"</div>
                <div class="hidden justify-center p-2 pb-0 font-bold lg:flex">"Wed"</div>
                <div class="hidden justify-center p-2 pb-0 font-bold lg:flex">"Thu"</div>
                <div class="hidden justify-center p-2 pb-0 font-bold lg:flex">"Fri"</div>
                <div class="hidden justify-center p-2 pb-0 font-bold lg:flex">"Sat"</div>
                <div class="hidden justify-center p-2 pb-0 font-bold lg:flex">"Sun"</div>
                <div class="hidden justify-center p-2 pb-0 font-bold lg:flex">"Week Avg/Total"</div>

                <Transition fallback=LoadingComponent>
                    <ErrorBoundary fallback=|errors| {
                        view! { <ErrorComponent errors/> }
                    }>{response}</ErrorBoundary>
                </Transition>
            </div>
        </main>
    }
}

#[component]
fn MonthListItem(data: MonthSummary, param_date: Signal<NaiveDate>) -> impl IntoView {
    let profile_href = data.get_profile_href();
    let diet_href = data.get_diet_day_href();
    let workout_href = data.get_workout_day_href();
    let progress_href = data.get_progress_detail_or_create_href();

    let calendar_date_title = data.get_calendar_date_title();

    let is_now = move || data.date == Utc::now().date_naive();
    let is_viewed = move || param_date.with(|value| *value == data.date);

    view! {
        <div class="bg-gray-100">
            <a
                class="font-bold block p-2 hover:bg-amber-200"
                class=("bg-blue-500", is_now)
                class=("bg-blue-400", is_viewed)
                class=("text-white", is_now() | is_viewed())
                class=("text-blue-500", !is_now() | !is_viewed())
                href=profile_href
            >
                {calendar_date_title}

            </a>

            <A class="block py-1 px-2 hover:bg-amber-200" href=diet_href>
                <div class="flex">
                    <BadgeDiet title="Calories" label="kcal" value=data.energy scale=0/>
                </div>
                <div class="flex">
                    <BadgeDiet title="Protein" label="g" value=data.protein scale=0/>
                    <BadgeDiet title="Carbs" label="g" value=data.carbohydrate scale=0/>
                    <BadgeDiet title="Fat" label="g" value=data.fat scale=0/>
                </div>
            </A>
            <A class="block py-1 px-2 hover:bg-amber-200" href=workout_href>
                <div class="flex">
                    <Badgei64 title="Workouts" value=data.workout_count/>
                </div>
                <div class="flex">
                    <Badgei64 title="Exercises" value=data.exercise_count/>
                    <Badgei64 title="Sets" value=data.set_count/>
                    <Badgei64 title="Reps" value=data.rep_count/>
                </div>
            </A>

            <A class="block py-1 px-2 mb-1 hover:bg-amber-200" href=progress_href>
                <div class="block space-y-1">
                    <BadgeProgress
                        title=""
                        label="kg"
                        value=data.weight.unwrap_or_default()
                        scale=1
                    />
                    <BadgeProgress
                        title=""
                        label="kcal"
                        value=data.energy_burnt.unwrap_or_default().into()
                        scale=0
                    />
                </div>
            </A>
        </div>
    }
}

#[component]
pub fn MonthListItemTotal(data: MonthSummary) -> impl IntoView {
    let week_title = data.get_week_title();
    let week_href = data.get_week_href();

    view! {
        <div class="bg-gray-300">
            <A class="block p-2 font-bold" href=week_href>
                {week_title}
            </A>

            <div class="block py-1 px-2">
                <div class="flex">
                    <BadgeDiet title="Calories" label="kcal" value=data.week_avg_energy scale=0/>
                </div>
                <div class="flex">
                    <BadgeDiet title="Protein" label="g" value=data.week_avg_protein scale=0/>
                    <BadgeDiet title="Carbs" label="g" value=data.week_avg_carbohydrate scale=0/>
                    <BadgeDiet title="Fat" label="g" value=data.week_avg_fat scale=0/>
                </div>
            </div>

            <div class="block py-1 px-2">
                <div class="flex">
                    <Badgei64 title="Workouts" value=data.week_total_workouts/>
                </div>
                <div class="flex">
                    <Badgei64 title="Exercises" value=data.week_total_exercises/>
                    <Badgei64 title="Sets" value=data.week_total_sets/>
                    <Badgei64 title="Reps" value=data.week_total_reps/>
                </div>
            </div>
            <div class="block py-1 px-2 mb-1 space-y-1">
                <BadgeProgress title="Avg Weight" label="kg" value=data.week_avg_weight scale=1/>
                <BadgeProgress
                    title="Avg Energy Burnt"
                    label="kcal"
                    value=data.week_avg_energy_burnt.into()
                    scale=0
                />

            </div>
        </div>
    }
}
