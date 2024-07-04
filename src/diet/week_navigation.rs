use chrono::prelude::*;
use leptos::server_fn::codec::GetUrl;
use leptos::*;

use crate::diet::model::DietWeekNav;

#[cfg(feature = "ssr")]
use {
    crate::util::datetime::{get_week_end, get_week_start},
    sqlx::Row,
};

#[cfg(feature = "ssr")]
use crate::{auth::model::User, auth::service::get_request_user, setup::get_pool};

#[server(endpoint = "user-diet-day-week-navigation", input = GetUrl)]
pub async fn get_diet_week_nav(
    username: String,
    date: NaiveDate,
) -> Result<Vec<DietWeekNav>, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    User::check_view_permission(&pool, &user, &username).await?;

    let start = get_week_start(date);
    let end = get_week_end(date);
    let query = sqlx::query(
        "
        WITH
        user_info AS (
            SELECT id FROM users_user WHERE username = $1
        ),
        week_series AS (
            SELECT
                ui.id AS user_id,
                DATE_TRUNC('day', dd)::date AS date
            FROM
                GENERATE_SERIES($2::TIMESTAMP, $3::TIMESTAMP, '1 day'::INTERVAL) AS dd
            CROSS JOIN user_info ui
        ),
        cte_diet_day_total AS (
            SELECT
                t1.user_id,
                t1.date,
                SUM(t1.quantity * t2.energy) AS energy,
                SUM(t1.quantity * t2.protein) AS protein,
                SUM(t1.quantity * t2.carbohydrate) AS carbohydrate,
                SUM(t1.quantity * t2.fat) AS fat
            FROM
                food_log t1
                LEFT JOIN food t2 ON t2.id = t1.food_id
            GROUP BY
                t1.user_id,
                t1.date
            )
        SELECT
            t1.date,
            COALESCE(t2.energy, 0) AS energy,
            COALESCE(t2.protein, 0) AS protein,
            COALESCE(t2.carbohydrate, 0) AS carbohydrate,
            COALESCE(t2.fat, 0) AS fat
        FROM
            week_series t1
            LEFT JOIN cte_diet_day_total t2 ON t2.user_id = t1.user_id AND t2.date = t1.date
        ",
    )
    .bind(&username)
    .bind(start)
    .bind(end)
    .fetch_all(&pool)
    .await?
    .iter()
    .map(|row| DietWeekNav {
        username: username.clone(),
        date: row.try_get("date").unwrap_or_default(),
        energy: row.try_get("energy").unwrap_or_default(),
        protein: row.try_get("protein").unwrap_or_default(),
        carbohydrate: row.try_get("carbohydrate").unwrap_or_default(),
        fat: row.try_get("fat").unwrap_or_default(),
    })
    .collect();
    Ok(query)
}

#[component]
pub fn DateItem(data: DietWeekNav) -> impl IntoView {
    let date_day_of_week = data.date.format("%a").to_string();
    let date_num = data.date.format("%d").to_string();
    let href = format!("/users/{}/diet/{}", data.username, data.date);
    view! {
        <a
            class="flex-1 p-2 text-center bg-blue-500 text-gray-100 hover:text-gray-900 hover:bg-amber-200"
            href=href
        >

            <h2>
                <span class="block md:inline-block">{date_day_of_week}</span>
                <span class="block md:inline-block md:ml-1">{date_num}</span>
            </h2>

            <div class="text-xs">
                <span class="block md:inline-block">{format!("{:.0}", data.energy)}</span>
                <span class="block md:inline-block md:ml-1">"kcal"</span>
            </div>
            <div class="hidden text-xs md:block">
                {format!("{:.0}g pro | ", data.protein)}
                {format!("{:.0}g carbs | ", data.carbohydrate)} {format!("{:.0}g fat", data.fat)}
            </div>
        </a>
    }
}
