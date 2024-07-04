use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use chrono::prelude::*;
use rust_decimal::prelude::*;

use super::model::MonthSummary;
use crate::chart::element::{Circle, Polyline};
use crate::chart::line::LineChart;
use crate::chart::util::map_value_to_range;
use crate::component::badge::{BadgePRO, BadgeWO, Badged};
use crate::component::date_navigation::DateNavigation;
use crate::component::template::{ErrorComponent, LoadingComponent};
use crate::util::datetime::{
    generate_month_range, get_month_end, get_month_start, get_week_start, Resolution,
};
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

    let stream = MonthSummary::get(&pool, &username, date).await?;

    Ok(stream)
}

#[component]
pub fn UserMonthPage() -> impl IntoView {
    let params = use_params_map();
    let username = move || get_username(&params);
    let date = move || get_date(&params);
    let month_resource = Resource::new(
        move || (username(), date()),
        |(username, date)| get_month_summary(username, date),
    );
    let month_response = move || {
        month_resource.and_then(|data| {
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

    let polyline_points = RwSignal::new(String::new());
    let y_min_value = 70.0;
    let y_max_value = 100.0;
    let x_axis_data = move || generate_month_range(date());
    let chart_data = move || {
        month_resource.and_then(|data| {
            polyline_points.update(|v| v.clear());

            let chart_left = 50.0;
            let chart_right = 1250.0;
            let chart_top = 50.0;
            let chart_bottom = 350.0;

            let first = get_month_start(date());
            let last = get_month_end(date());

            let x_min_value = first.day() as f64;
            let x_max_value = last.day() as f64;

            data.iter()
                .filter(|x| x.date >= first && x.date <= last)
                .map(|inner| {
                    let data_point = inner.weight.map(|weight| {
                        let date_f64 = inner.date.day() as f64;
                        let weight_f64 = weight.to_f64().unwrap_or_default();

                        let x = map_value_to_range(
                            date_f64,
                            x_min_value,
                            x_max_value,
                            chart_left,
                            chart_right,
                        );
                        let y = map_value_to_range(
                            weight_f64,
                            y_min_value,
                            y_max_value,
                            chart_bottom,
                            chart_top,
                        );
                        // leptos::logging::log!("x: {}, y: {}", x, y);
                        polyline_points.update(|v| *v = format!("{}{},{} ", v, x, y));
                        view! { <Circle cx=x cy=y r=5.0/> }
                    });

                    data_point
                })
                .collect_view()
        })
    };

    view! {
        <Title text="Month"/>
        <main class="p-4 m-4 bg-white border">
            <div class="pb-4">
                <DateNavigation resolution=Resolution::Month/>
            </div>

            <div class="flex overflow-x-auto p-4 border border-amber-400 snap-x snap-mandatory">
                <div class="mx-auto min-w-[1300px]">
                    <Transition>
                        <LineChart
                            y_min=y_min_value
                            y_max=y_max_value
                            x_axis_data=Signal::derive(x_axis_data)
                        >
                            <Polyline
                                points=polyline_points
                                attr:fill="none"
                                attr:stroke="3"
                                attr:stroke-dasharray="2000"
                                attr:stroke-dashoffset="2000"
                                attr:class="stroke-red-500 stroke-3 animate-line"
                            />
                            {chart_data}
                        </LineChart>
                    </Transition>
                </div>
            </div>
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
                    }>{month_response}</ErrorBoundary>
                </Transition>
            </div>
        </main>
    }
}

#[component]
fn MonthListItem(data: MonthSummary, param_date: Signal<NaiveDate>) -> impl IntoView {
    let profile_href = format!("/users/{}/{}", data.username, data.date);
    let diet_href = format!("/users/{}/diet/{}", data.username, data.date);
    let workout_href = format!("/users/{}/workouts/{}", data.username, data.date);
    let progress_href = match data.progress_date {
        Some(d) => format!("/users/{}/progress/{}", data.username, d),
        None => format!(
            "/users/{}/progress?progress-create-date={}",
            data.username, data.date
        ),
    };
    let now = Utc::now().date_naive();
    view! {
        <div class="bg-gray-200">
            <h2
                class="font-bold"
                class=("bg-blue-500", move || data.date == now)
                class=("text-white", move || data.date == now)
                class=("text-blue-500", move || data.date != now)
                class=("bg-blue-400", move || data.date == param_date.get_untracked())
                class=("text-white", move || data.date == param_date.get_untracked())
            >
                <A href=profile_href class="block p-2 hover:bg-amber-200">
                    {data.date.format("%a %d").to_string()}
                </A>
            </h2>
            <A class="block py-1 px-2 hover:bg-amber-200" href=diet_href>
                <div class="flex">
                    <Badged title="Calories" label="kcal" value=data.energy scale=0/>
                </div>
                <div class="flex">
                    <Badged title="Protein" label="g" value=data.protein scale=0/>
                    <Badged title="Carbs" label="g" value=data.carbohydrate scale=0/>
                    <Badged title="Fat" label="g" value=data.fat scale=0/>
                </div>
            </A>
            <A class="block py-1 px-2 hover:bg-amber-200" href=workout_href>
                <div class="flex">
                    <BadgeWO title="Workouts" value=data.workout_count/>
                </div>
                <div class="flex">
                    <BadgeWO title="Exercises" value=data.exercise_count/>
                    <BadgeWO title="Sets" value=data.set_count/>
                    <BadgeWO title="Reps" value=data.rep_count/>
                </div>
            </A>

            <A class="block py-1 px-2 mb-1 hover:bg-amber-200" href=progress_href>
                <div class="block space-y-1">
                    <BadgePRO title="" label="kg" value=data.weight.unwrap_or_default() scale=1/>
                    <BadgePRO
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
    let week_title = format!(
        "Week {}, {}",
        data.date.iso_week().week(),
        data.date.format("%Y")
    );
    let week = get_week_start(data.date);
    let week_href = format!("/users/{}/week/{}", data.username, week);

    view! {
        <div class="bg-gray-300">
            <A class="block p-2 font-bold" href=week_href>
                {week_title}
            </A>

            <div class="block py-1 px-2">
                <div class="flex">
                    <Badged title="Calories" label="kcal" value=data.week_avg_energy scale=0/>
                </div>
                <div class="flex">
                    <Badged title="Protein" label="g" value=data.week_avg_protein scale=0/>
                    <Badged title="Carbs" label="g" value=data.week_avg_carbohydrate scale=0/>
                    <Badged title="Fat" label="g" value=data.week_avg_fat scale=0/>
                </div>
            </div>

            <div class="block py-1 px-2">
                <div class="flex">
                    <BadgeWO title="Workouts" value=data.week_total_workouts/>
                </div>
                <div class="flex">
                    <BadgeWO title="Exercises" value=data.week_total_exercises/>
                    <BadgeWO title="Sets" value=data.week_total_sets/>
                    <BadgeWO title="Reps" value=data.week_total_reps/>
                </div>
            </div>
            <div class="block py-1 px-2 mb-1 space-y-1">
                <BadgePRO title="Avg Weight" label="kg" value=data.week_avg_weight scale=1/>
                <BadgePRO
                    title="Avg Energy Burnt"
                    label="kcal"
                    value=data.week_avg_energy_burnt.into()
                    scale=0
                />

            </div>
        </div>
    }
}
