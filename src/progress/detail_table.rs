use leptos::*;

use crate::util::datetime::{format_datetime, DATE_FORMAT_SHORT};

use super::model::ProgressQuery;

#[component]
pub fn ProgressDetailTable(data: ProgressQuery) -> impl IntoView {
    let date = data.date.format(DATE_FORMAT_SHORT).to_string();

    let weight = data
        .weight
        .map_or_else(|| "-".to_string(), |res| format!("{:.2} kg", res));
    let week_avg_weight = data
        .week_avg_weight
        .map_or_else(|| "-".to_string(), |res| format!("{:.2} kg", res));
    let month_avg_weight = data
        .month_avg_weight
        .map_or_else(|| "-".to_string(), |res| format!("{:.2} kg", res));

    let energy_burnt = data
        .energy_burnt
        .map_or_else(|| "-".to_string(), |res| format!("{} kcal", res));
    let week_avg_energy_burnt = data
        .week_avg_energy_burnt
        .map_or_else(|| "-".to_string(), |res| format!("{} kcal", res));
    let month_avg_energy_burnt = data
        .month_avg_energy_burnt
        .map_or_else(|| "-".to_string(), |res| format!("{} kcal", res));

    let notes = data.notes.map_or_else(|| "-".to_string(), |res| res);

    let created_at = format_datetime(&Some(data.created_at));
    let updated_at = format_datetime(&data.updated_at);

    view! {
        <div>
            <table class="w-full border-collapse table-fixed">
                <thead>
                    <tr>
                        <th class="p-2 w-1/2 text-start">"Date"</th>
                        <th class="p-2 w-1/2 text-end">{date}</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td class="p-2 w-1/2 text-left border">
                            <div>"Weight"</div>
                            <div class="text-xs text-gray-500">"Week Average"</div>
                            <div class="text-xs text-gray-500">"Month Average"</div>
                        </td>
                        <td class="p-2 w-1/2 text-right border">
                            <div>{weight}</div>
                            <div class="text-xs text-gray-500">{week_avg_weight}</div>
                            <div class="text-xs text-gray-500">{month_avg_weight}</div>
                        </td>
                    </tr>
                    <tr>
                        <td class="p-2 w-1/2 text-left border">
                            <div>"Energy Burnt"</div>
                            <div class="text-xs text-gray-500">"Week Average"</div>
                            <div class="text-xs text-gray-500">"Month Average"</div>
                        </td>
                        <td class="p-2 w-1/2 text-right border">
                            <div>{energy_burnt}</div>
                            <div class="text-xs text-gray-500">{week_avg_energy_burnt}</div>
                            <div class="text-xs text-gray-500">{month_avg_energy_burnt}</div>
                        </td>
                    </tr>
                    <tr>
                        <td class="p-2 w-1/2 text-left border">"Notes"</td>
                        <td class="p-2 w-1/2 text-right border">{notes}</td>
                    </tr>
                    <tr>
                        <td class="p-2 w-1/2 text-left border">"Created"</td>
                        <td class="p-2 w-1/2 text-right border">{created_at}</td>
                    </tr>
                    <tr>
                        <td class="p-2 w-1/2 text-left border">"Updated"</td>
                        <td class="p-2 w-1/2 text-right border">{updated_at}</td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}
