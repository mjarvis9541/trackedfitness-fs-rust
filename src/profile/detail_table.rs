use leptos::*;

use crate::profile::model::ProfileMetric;
use crate::util::datetime::{format_datetime, DATE_FORMAT_SHORT};

#[component]
pub fn ProfileDetailTable(data: ProfileMetric) -> impl IntoView {
    let created_at = format_datetime(&Some(data.created_at));
    let updated_at = format_datetime(&data.updated_at);

    let latest_weight = data
        .latest_weight
        .map_or_else(|| "#".to_string(), |weight| format!("{:.2}kg", weight));
    let latest_weight_date = data.latest_weight_date.map_or_else(
        || "-".to_string(),
        |d| d.format(DATE_FORMAT_SHORT).to_string(),
    );
    let latest_weight_href = data.latest_weight_date.map_or_else(
        || "#".to_string(),
        |date| format!("/users/{}/progress/{}", data.username, date),
    );

    view! {
        <div>
            <table class="w-full border-collapse table-fixed">
                <tbody>
                    <tr>
                        <td class="p-2 w-1/2 text-left border">"Height"</td>
                        <td class="p-2 w-1/2 text-right border">
                            {format!("{:.*} cm", 2, data.height)}
                        </td>
                    </tr>
                    <tr>
                        <td class="p-2 w-1/2 text-left border">"Weight"</td>
                        <td class="p-2 w-1/2 text-right border">
                            <a href=latest_weight_href class="hover:underline">
                                {latest_weight}
                                <div class="text-xs text-gray-500">{latest_weight_date}</div>
                            </a>
                        </td>
                    </tr>
                    <tr>
                        <td class="p-2 w-1/2 text-left border">"Sex"</td>
                        <td class="p-2 w-1/2 text-right border">{data.sex_display}</td>
                    </tr>
                    <tr>
                        <td class="p-2 w-1/2 text-left border">"Age"</td>
                        <td class="p-2 w-1/2 text-right border">{data.age}</td>
                    </tr>
                    <tr>
                        <td class="p-2 w-1/2 text-left border">
                            <a href="/help#bmi" class="text-blue-500 §hover:underline">
                                "Body Mass Index (BMI)"
                            </a>
                        </td>
                        <td class="p-2 w-1/2 text-right border">
                            {format!("{:.2}", data.body_mass_index)}
                        </td>
                    </tr>
                    <tr>
                        <td class="p-2 w-1/2 text-left border">
                            <a href="/help#bmr" class="text-blue-500 §hover:underline">
                                "Basal Metabolic Rate (BMR)"
                            </a>
                        </td>
                        <td class="p-2 w-1/2 text-right border">
                            {format!("{:.*} kcal", 0, data.basal_metabolic_rate)}
                        </td>
                    </tr>
                    <tr>
                        <td class="p-2 w-1/2 text-left border">"Activity Level"</td>
                        <td class="p-2 w-1/2 text-right border">{data.activity_level_display}</td>
                    </tr>
                    <tr>
                        <td class="p-2 w-1/2 text-left border">
                            <a href="/help#tdee" class="text-blue-500 §hover:underline">
                                "Total Daily Energy Expenditure (TDEE)"
                            </a>
                        </td>
                        <td class="p-2 w-1/2 text-right border">
                            {format!("{:.*} kcal", 0, data.total_daily_energy_expenditure)}
                        </td>
                    </tr>
                    <tr>
                        <td class="p-2 w-1/2 text-left border">"Fitness Goal"</td>
                        <td class="p-2 w-1/2 text-right border">{data.fitness_goal_display}</td>
                    </tr>
                    <tr>
                        <td class="p-2 w-1/2 text-left border">"Target Calories"</td>
                        <td class="p-2 w-1/2 text-right border">
                            {format!("{:.*} kcal", 0, data.target_calories)}
                        </td>
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
