pub fn generate_labels(y_min: f64, y_max: f64, num_points: f64) -> Vec<String> {
    let step = (y_max - y_min) / (num_points - 1.0);

    (0..num_points as usize)
        .map(|i| {
            let value = y_max - (step * i as f64);
            value.round().to_string()
        })
        .collect()
}

// pub fn generate_labels_generic<T>(min_value: T, max_value: T, num_points: usize) -> Vec<String>
// where
//     T: Into<Decimal> + Copy,
// {
//     let min_value_dec = Decimal::from_f64(min_value.into().to_f64().unwrap()).unwrap();
//     let max_value_dec = Decimal::from_f64(max_value.into().to_f64().unwrap()).unwrap();

//     let step = (max_value_dec - min_value_dec) / Decimal::from_usize(num_points - 1).unwrap();

//     (0..num_points)
//         .map(|i| {
//             let value = min_value_dec + (step * Decimal::from_usize(i).unwrap());
//             format!("{:.0}", value)
//         })
//         .collect()
// }

// pub fn calculate_chart_length(canvas_length: usize, padding_1: usize, padding_2: usize) -> usize {
//     canvas_length.saturating_sub(padding_1 + padding_2)
// }

// pub fn calculate_interval(chart_length: usize, num_points: usize) -> f64 {
//     chart_length as f64 / (num_points - 1) as f64
// }

// pub fn calculate_position(chart_start: f64, interval: f64, index: f64) -> f64 {
//     chart_start as f64 + (index as f64 * interval)
// }

// pub fn calculate_x_position(
//     index: f64,
//     num_points: f64,
//     chart_left: f64,
//     chart_right: f64,
// ) -> usize {
//     let chart_width = chart_right - chart_left;
//     (chart_left + index / (num_points - 1.0) * chart_width) as usize
// }

// pub fn calculate_y_position(
//     value: f64,
//     y_min: f64,
//     y_max: f64,
//     chart_top: f64,
//     chart_bottom: f64,
// ) -> usize {
//     let y_range = y_max - y_min;
//     let chart_height = chart_bottom - chart_top;

//     let relative_value = (value - y_min) / y_range;
//     (chart_bottom - (relative_value * chart_height)) as usize
// }

/// Calculate the position using linear interpolation
pub fn map_value_to_range(value: f64, min_value: f64, max_value: f64, start: f64, end: f64) -> f64 {
    start + (value - min_value) / (max_value - min_value) * (end - start)
}
