use std::collections::HashSet;

use chrono::NaiveDate;
use uuid::Uuid;

use crate::config::get_config;

use crate::util::datetime::DATE_FORMAT_ISO;

pub fn slugify(s: &str) -> String {
    let config = get_config();

    let mut slug = s.to_lowercase();
    slug = config.slug_regex.replace_all(&slug, "-").to_string();

    slug.trim_matches('-').to_string()
}

pub fn normalize_whitespace(s: &str) -> String {
    s.trim().split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn parse_uuids_from_strings(items: &HashSet<String>) -> Result<Vec<Uuid>, uuid::Error> {
    items.iter().map(|id| Uuid::parse_str(id)).collect()
}

pub fn parse_dates_from_strings(
    items: &HashSet<String>,
) -> Result<Vec<NaiveDate>, chrono::ParseError> {
    items
        .iter()
        .map(|item| NaiveDate::parse_from_str(item, DATE_FORMAT_ISO))
        .collect()
}
