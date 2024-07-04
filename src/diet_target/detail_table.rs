use leptos::*;

use crate::util::datetime::{format_datetime, DATE_FORMAT_SHORT};

use super::model::DietTarget;

#[component]
pub fn DietTargetDetailTable(data: DietTarget) -> impl IntoView {
    let date = data.date.format(DATE_FORMAT_SHORT).to_string();

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
                        <td class="p-2 w-1/2 text-left border">"Weight"</td>
                        <td class="p-2 w-1/2 text-right border">
                            {format!("{:.2}", data.weight)} " kg"
                        </td>
                    </tr>
                    <tr>
                        <td class="p-2 text-left border">"Calories"</td>
                        <td class="p-2 text-right border">{data.energy} " kcal"</td>
                    </tr>
                    <tr>
                        <td class="p-2 text-left border">"Protein"</td>
                        <td class="p-2 text-right border">
                            {format!("{:.1}", data.protein)} "g"
                            <span class="text-xs text-gray-400">
                                " (" {format!("{:.2}", data.protein_pct)} "%)"
                            </span>
                        </td>
                    </tr>
                    <tr>
                        <td class="p-2 text-left border">"Carbohydrate"</td>
                        <td class="p-2 text-right border">
                            {format!("{:.1}", data.carbohydrate)} "g"
                            <span class="text-xs text-gray-400">
                                " (" {format!("{:.2}", data.carbohydrate_pct)} "%)"
                            </span>
                        </td>
                    </tr>
                    <tr>
                        <td class="p-2 text-left border">"Fat"</td>
                        <td class="p-2 text-right border">
                            {format!("{:.1}", data.fat)} "g"
                            <span class="text-xs text-gray-400">
                                " (" {format!("{:.2}", data.fat_pct)} "%)"
                            </span>
                        </td>
                    </tr>
                    <tr>
                        <td class="p-2 text-left border">"Saturates"</td>
                        <td class="p-2 text-right border">
                            {format!("{:.1}", data.saturates)} "g"
                        </td>
                    </tr>
                    <tr>
                        <td class="p-2 text-left border">"Sugars"</td>
                        <td class="p-2 text-right border">{format!("{:.1}", data.sugars)} "g"</td>
                    </tr>
                    <tr>
                        <td class="p-2 text-left border">"Fibre"</td>
                        <td class="p-2 text-right border">{format!("{:.1}", data.fibre)} "g"</td>
                    </tr>
                    <tr>
                        <td class="p-2 text-left border">"Salt"</td>
                        <td class="p-2 text-right border">{format!("{:.1}", data.salt)} "g"</td>
                    </tr>
                    <tr>
                        <td class="p-2 text-left border">"Calories per kg"</td>
                        <td class="p-2 text-right border">
                            {format!("{:.0}", data.energy_per_kg)} "kcal"
                        </td>
                    </tr>
                    <tr>
                        <td class="p-2 text-left border">"Protein per kg"</td>
                        <td class="p-2 text-right border">
                            {format!("{:.2}", data.protein_per_kg)} "g"
                        </td>
                    </tr>
                    <tr>
                        <td class="p-2 text-left border">"Carbohydrate per kg"</td>
                        <td class="p-2 text-right border">
                            {format!("{:.2}", data.carbohydrate_per_kg)} "g"
                        </td>
                    </tr>
                    <tr>
                        <td class="p-2 text-left border">"Fat per kg"</td>
                        <td class="p-2 text-right border">
                            {format!("{:.2}", data.fat_per_kg)} "g"
                        </td>
                    </tr>
                    <tr>
                        <td class="p-2 text-left border">"Created"</td>
                        <td class="p-2 text-right border">{created_at}</td>
                    </tr>
                    <tr>
                        <td class="p-2 text-left border">"Updated"</td>
                        <td class="p-2 text-right border">{updated_at}</td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}
