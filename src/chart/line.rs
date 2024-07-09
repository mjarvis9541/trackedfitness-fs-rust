use leptos::*;

use crate::chart::element::{Line, Text};
use crate::chart::util::{generate_labels, map_value_to_range};

#[component]
pub fn ChartAxisX(#[prop(optional, into)] x_axis_data: MaybeSignal<Vec<String>>) -> impl IntoView {
    let chart_left = 50.0;
    let chart_right = 1250.0;
    let chart_top = 50.0;
    let chart_bottom = 350.0;

    let x_axis_label_y = 375.0;

    let x_axis_view = move || {
        let x_min_value = 1.0;
        let x_max_value = x_axis_data.with(|x| x.len() as f64);

        x_axis_data()
            .into_iter()
            .enumerate()
            .map(|(i, label)| {
                let x = map_value_to_range(
                    (i + 1) as f64,
                    x_min_value,
                    x_max_value,
                    chart_left,
                    chart_right,
                );
                view! {
                    <Line
                        x1=x
                        y1=chart_top
                        x2=x
                        y2=chart_bottom
                        attr:stroke-1
                        attr:stroke-dasharray="5, 5"
                    />

                    <Text x=x y=x_axis_label_y text=label attr:text-anchor="middle"/>
                }
            })
            .collect_view()
    };
    x_axis_view
}

#[component]
pub fn ChartAxisY(
    #[prop(optional, into)] y_min: MaybeSignal<f64>,
    #[prop(optional, into)] y_max: MaybeSignal<f64>,
    #[prop(optional, into)] y_num_points: MaybeSignal<f64>,
) -> impl IntoView {
    let chart_left = 50.0;
    let chart_right = 1250.0;
    let chart_top = 50.0;
    let chart_bottom = 350.0;

    let y_axis_min = y_min.get();
    let y_axis_max = y_max.get();
    let y_axis_num_points = y_num_points.get();
    let y_axis_num_points = if y_axis_num_points == 0.0 {
        7.0
    } else {
        y_axis_num_points
    };
    let y_axis_labels = move || generate_labels(y_axis_min, y_axis_max, y_axis_num_points);

    // leptos::logging::log!("{:?}", y_axis_labels());
    let y_axis_label_x = 25.0;
    let y_axis_view = move || {
        y_axis_labels()
            .into_iter()
            .enumerate()
            .map(|(i, label)| {
                let y = map_value_to_range(
                    i as f64,
                    0.0,
                    y_axis_num_points - 1.0,
                    chart_top,
                    chart_bottom,
                );

                view! {
                    <Line x1=chart_left y1=y x2=chart_right y2=y attr:stroke-dasharray="5, 5"/>
                    <Text x=y_axis_label_x y=y text=label attr:text-anchor="middle"/>
                }
            })
            .collect_view()
    };
    y_axis_view
}

#[component]
pub fn LineChart(
    #[prop(optional, into)] x_axis_data: MaybeSignal<Vec<String>>,
    #[prop(optional, into)] y_min: MaybeSignal<f64>,
    #[prop(optional, into)] y_max: MaybeSignal<f64>,
    #[prop(optional, into)] y_num_points: MaybeSignal<f64>,
    children: Children,
) -> impl IntoView {
    view! {
        <svg
            width="100%"
            height="100%"
            viewBox="0 0 1300 400"
            preserveAspectRatio="xMidYMid meet"
            class="bg-gray-100 border"
        >
            <ChartAxisX x_axis_data/>
            <ChartAxisY y_min y_max y_num_points/>
            {children()}
        </svg>
    }
}
