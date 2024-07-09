use std::collections::HashMap;

use chrono::{NaiveDate, TimeDelta};
use rust_decimal::prelude::*;

use crate::error::Result;

use super::model::UserDaySummary;

impl UserDaySummary {
    pub fn fill_missing_days_with_previous_with_default(
        summaries: Vec<UserDaySummary>,
        start_date: NaiveDate,
        end_date: NaiveDate,
        username: &str,
    ) -> Result<Vec<UserDaySummary>> {
        let mut complete_data: Vec<UserDaySummary> = Vec::new();

        let summary_map: HashMap<NaiveDate, UserDaySummary> =
            summaries.into_iter().map(|s| (s.date, s)).collect();

        for day in 0..=(end_date - start_date).num_days() {
            let current_date = start_date + TimeDelta::days(day);

            if let Some(entry) = summary_map.get(&current_date) {
                complete_data.push(entry.clone());
            } else {
                let new_entry = UserDaySummary {
                    username: username.to_string(),
                    date: current_date,
                    actual: false,
                    ..Default::default()
                };
                complete_data.push(new_entry);
            }
        }

        Ok(complete_data)
    }

    pub fn fill_missing_days_with_previous(
        summaries: Vec<UserDaySummary>,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<UserDaySummary>> {
        let mut complete_data: Vec<UserDaySummary> = Vec::new();
        let mut last_entry: Option<UserDaySummary> = None;

        for day in 0..=(end_date - start_date).num_days() {
            let current_date = start_date + TimeDelta::days(day);

            if let Some(entry) = summaries.iter().find(|e| e.date == current_date) {
                complete_data.push(entry.clone());
                last_entry = Some(entry.clone());
            } else if let Some(last_entry) = last_entry.clone() {
                let mut new_entry = last_entry;
                new_entry.date = current_date;
                new_entry.actual = false;
                complete_data.push(new_entry);
            } else {
                let new_entry = UserDaySummary {
                    date: current_date,
                    ..Default::default()
                };
                complete_data.push(new_entry);
            }
        }
        Ok(complete_data)
    }

    pub fn calculate_averages(
        day_summaries: &[UserDaySummary],
        total_days: i32,
    ) -> Result<UserDaySummary> {
        if total_days == 0 {
            return Ok(UserDaySummary::default());
        }

        let mut avg_summary = UserDaySummary::default();
        for day_summary in day_summaries {
            avg_summary.add(day_summary);
        }

        let divisor = Decimal::from_i32(total_days).unwrap_or_default();
        avg_summary.divide(divisor)?;

        Ok(avg_summary)
    }
}
