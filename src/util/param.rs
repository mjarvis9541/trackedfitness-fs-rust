use std::error::Error;

use leptos::*;
use leptos_router::*;

use chrono::{NaiveDate, Utc};
use uuid::Uuid;

use crate::util::datetime::{parse_date, Resolution};

pub fn update_date_in_url_path(url: &str, new_date: &str) -> Result<String, Box<dyn Error>> {
    let mut parts: Vec<&str> = url.split('/').collect();
    if let Some(part) = parts
        .iter_mut()
        .find(|part| NaiveDate::parse_from_str(part, "%Y-%m-%d").is_ok())
    {
        *part = new_date;
    } else {
        parts.push(new_date);
    }
    Ok(parts.join("/"))
}

pub fn get_next_date_url(
    resolution: &Resolution,
    path: &str,
    date: NaiveDate,
) -> Result<String, Box<dyn Error>> {
    let new_date = resolution.get_next_date(date);
    update_date_in_url_path(path, &new_date.to_string())
}

pub fn get_previous_date_url(
    resolution: &Resolution,
    path: &str,
    date: NaiveDate,
) -> Result<String, Box<dyn Error>> {
    let new_date = resolution.get_previous_date(date);
    update_date_in_url_path(path, &new_date.to_string())
}

pub fn get_current_date_url(path: &str) -> Result<String, Box<dyn Error>> {
    let new_date = Utc::now().date_naive();
    update_date_in_url_path(path, &new_date.to_string())
}

fn get_param_value(params: &Memo<ParamsMap>, key: &str) -> Option<String> {
    params.with(|q| q.get(key).cloned())
}

pub fn get_slug(params: &Memo<ParamsMap>) -> String {
    get_param_value(params, "slug").unwrap_or_default()
}

pub fn get_username(params: &Memo<ParamsMap>) -> String {
    get_param_value(params, "username").unwrap_or_default()
}

pub fn get_date(params: &Memo<ParamsMap>) -> NaiveDate {
    get_param_value(params, "date")
        .map_or_else(|| Utc::now().date_naive(), |date_str| parse_date(&date_str))
}

pub fn extract_param(query: &Memo<ParamsMap>, key: &str) -> String {
    get_param_value(query, key).unwrap_or_default()
}

pub fn extract_size(query: &Memo<ParamsMap>) -> i64 {
    query.with(|q| {
        q.get("size")
            .and_then(|size| size.parse::<i64>().ok())
            .filter(|size| matches!(*size, 10 | 25 | 50 | 75 | 100))
            .unwrap_or(25)
    })
}
pub fn extract_page(query: &Memo<ParamsMap>) -> i64 {
    query.with(|q| {
        q.get("page")
            .and_then(|page| page.parse::<i64>().ok())
            .filter(|page| *page > 0)
            .unwrap_or(1)
    })
}

#[derive(Debug, PartialEq, Params)]
pub struct UuidParam {
    pub id: Uuid,
}

#[derive(Debug, PartialEq, Params)]
pub struct UsernameParam {
    pub username: String,
}

#[derive(Debug, PartialEq, Params)]
pub struct UsernameDateParam {
    pub username: String,
    pub date: NaiveDate,
}

pub fn generate_create_workout_url(params: &Memo<ParamsMap>) -> String {
    params.with(|q| {
        format!(
            "/users/{}/workouts/{}/create-with-exercise",
            q.get("username").cloned().unwrap_or_default(),
            q.get("date")
                .map_or_else(|| Utc::now().date_naive(), |date_str| parse_date(&date_str))
        )
    })
}
