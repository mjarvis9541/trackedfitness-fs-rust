use std::mem;

use chrono::{DateTime, Datelike, Months, NaiveDate, TimeDelta, Utc, Weekday};

pub const DATE_FORMAT_ISO: &str = "%Y-%m-%d";
pub const DATE_FORMAT_LONG: &str = "%A %d %B %Y";
pub const DATE_FORMAT_SHORT: &str = "%a %d %b %Y";
pub const DATETIME_FORMAT_SHORT: &str = "%a %d %b %Y, %H:%M:%S";

#[derive(Debug, Clone, Copy)]
pub enum Resolution {
    Day,
    Week,
    Month,
}

impl Resolution {
    pub fn get_next_date(&self, date: NaiveDate) -> NaiveDate {
        match self {
            Resolution::Day => date + TimeDelta::days(1),
            Resolution::Week => date + TimeDelta::weeks(1),
            Resolution::Month => date + Months::new(1),
        }
    }

    pub fn get_previous_date(&self, date: NaiveDate) -> NaiveDate {
        match self {
            Resolution::Day => date - TimeDelta::days(1),
            Resolution::Week => date - TimeDelta::weeks(1),
            Resolution::Month => date - Months::new(1),
        }
    }
}

#[derive(Debug)]
pub struct DateRange(pub NaiveDate, pub NaiveDate);

impl Iterator for DateRange {
    type Item = NaiveDate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 <= self.1 {
            let next = self.0 + TimeDelta::days(1);
            Some(mem::replace(&mut self.0, next))
        } else {
            None
        }
    }
}

pub trait NaiveDateExt {
    fn days_in_month(&self) -> u32;
    // fn days_in_year(&self) -> u32;
    fn is_leap_year(&self) -> bool;
}

impl NaiveDateExt for NaiveDate {
    fn days_in_month(&self) -> u32 {
        let month = self.month();
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.is_leap_year() {
                    29
                } else {
                    28
                }
            }
            _ => panic!("Invalid month: {}", month),
        }
    }

    // fn days_in_year(&self) -> u32 {
    //     if self.is_leap_year() {
    //         366
    //     } else {
    //         365
    //     }
    // }

    fn is_leap_year(&self) -> bool {
        let year = self.year();
        year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
    }
}

pub fn get_week_start(date: NaiveDate) -> NaiveDate {
    NaiveDate::from_isoywd_opt(date.iso_week().year(), date.iso_week().week(), Weekday::Mon)
        .expect("Invalid date")
}

pub fn get_week_end(date: NaiveDate) -> NaiveDate {
    NaiveDate::from_isoywd_opt(date.iso_week().year(), date.iso_week().week(), Weekday::Sun)
        .expect("Invalid date")
}

pub fn get_month_start(date: NaiveDate) -> NaiveDate {
    NaiveDate::from_ymd_opt(date.year(), date.month(), 1).expect("Invalid date")
}

pub fn get_month_end(date: NaiveDate) -> NaiveDate {
    NaiveDate::from_ymd_opt(date.year(), date.month(), date.days_in_month()).expect("Invalid date")
}

pub fn get_month_start_comprehensive(date: NaiveDate) -> NaiveDate {
    let start = get_month_start(date);
    if start.weekday() == Weekday::Mon {
        start
    } else {
        get_week_start(start)
    }
}

pub fn get_month_end_comprehensive(date: NaiveDate) -> NaiveDate {
    let end = get_month_end(date);
    if end.weekday() == Weekday::Sun {
        end
    } else {
        get_week_end(end)
    }
}

// pub fn generate_week_range(date: NaiveDate) -> HashSet<String> {
//     let start = get_week_start(date);
//     let end = get_week_end(date);
//     DateRange(start, end).map(|date| date.to_string()).collect()
// }

pub fn generate_month_range(date: NaiveDate) -> Vec<String> {
    let start = get_month_start(date);
    let end = get_month_end(date);
    DateRange(start, end)
        .map(|d| d.format("%d").to_string())
        .collect()
}

// pub fn generate_month_range_comprehensive(date: NaiveDate) -> Vec<NaiveDate> {
//     let start = get_month_start_comprehensive(date);
//     let end = get_month_end_comprehensive(date);
//     DateRange(start, end).collect()
// }

pub fn format_datetime(date: &Option<DateTime<Utc>>) -> String {
    match date {
        Some(date) => date.format(DATETIME_FORMAT_SHORT).to_string(),
        None => "-".to_string(),
    }
}

pub fn parse_date(date_str: &str) -> NaiveDate {
    NaiveDate::parse_from_str(date_str, DATE_FORMAT_ISO).unwrap_or_else(|_| Utc::now().date_naive())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, Utc};

    #[test]
    fn test_resolution_get_next_date() {
        let date = NaiveDate::from_ymd_opt(2024, 6, 29).unwrap();
        assert_eq!(
            Resolution::Day.get_next_date(date),
            NaiveDate::from_ymd_opt(2024, 6, 30).unwrap()
        );
        assert_eq!(
            Resolution::Week.get_next_date(date),
            NaiveDate::from_ymd_opt(2024, 7, 6).unwrap()
        );
        assert_eq!(
            Resolution::Month.get_next_date(date),
            NaiveDate::from_ymd_opt(2024, 7, 29).unwrap()
        );
    }

    #[test]
    fn test_resolution_get_previous_date() {
        let date = NaiveDate::from_ymd_opt(2024, 6, 29).unwrap();
        assert_eq!(
            Resolution::Day.get_previous_date(date),
            NaiveDate::from_ymd_opt(2024, 6, 28).unwrap()
        );
        assert_eq!(
            Resolution::Week.get_previous_date(date),
            NaiveDate::from_ymd_opt(2024, 6, 22).unwrap()
        );
        assert_eq!(
            Resolution::Month.get_previous_date(date),
            NaiveDate::from_ymd_opt(2024, 5, 29).unwrap()
        );
    }

    #[test]
    fn test_date_range_iterator() {
        let start_date = NaiveDate::from_ymd_opt(2024, 6, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2024, 6, 5).unwrap();
        let mut date_range = DateRange(start_date, end_date);

        assert_eq!(
            date_range.next(),
            Some(NaiveDate::from_ymd_opt(2024, 6, 1).unwrap())
        );
        assert_eq!(
            date_range.next(),
            Some(NaiveDate::from_ymd_opt(2024, 6, 2).unwrap())
        );
        assert_eq!(
            date_range.next(),
            Some(NaiveDate::from_ymd_opt(2024, 6, 3).unwrap())
        );
        assert_eq!(
            date_range.next(),
            Some(NaiveDate::from_ymd_opt(2024, 6, 4).unwrap())
        );
        assert_eq!(
            date_range.next(),
            Some(NaiveDate::from_ymd_opt(2024, 6, 5).unwrap())
        );
        assert_eq!(date_range.next(), None);
    }

    #[test]
    fn test_days_in_month() {
        let date = NaiveDate::from_ymd_opt(2024, 6, 29).unwrap();
        assert_eq!(date.days_in_month(), 30);

        let date = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        assert_eq!(date.days_in_month(), 29);

        let date = NaiveDate::from_ymd_opt(2023, 2, 28).unwrap();
        assert_eq!(date.days_in_month(), 28);
    }

    #[test]
    fn test_is_leap_year() {
        let date = NaiveDate::from_ymd_opt(2024, 6, 29).unwrap();
        assert!(date.is_leap_year());

        let date = NaiveDate::from_ymd_opt(2023, 6, 29).unwrap();
        assert!(!date.is_leap_year());

        let date = NaiveDate::from_ymd_opt(2000, 6, 29).unwrap();
        assert!(date.is_leap_year());

        let date = NaiveDate::from_ymd_opt(1900, 6, 29).unwrap();
        assert!(!date.is_leap_year());
    }

    #[test]
    fn test_get_week_start() {
        let date = NaiveDate::from_ymd_opt(2024, 6, 29).unwrap();
        assert_eq!(
            get_week_start(date),
            NaiveDate::from_ymd_opt(2024, 6, 24).unwrap()
        );
    }

    #[test]
    fn test_get_week_end() {
        let date = NaiveDate::from_ymd_opt(2024, 6, 29).unwrap();
        assert_eq!(
            get_week_end(date),
            NaiveDate::from_ymd_opt(2024, 6, 30).unwrap()
        );
    }

    #[test]
    fn test_get_month_start() {
        let date = NaiveDate::from_ymd_opt(2024, 6, 29).unwrap();
        assert_eq!(
            get_month_start(date),
            NaiveDate::from_ymd_opt(2024, 6, 1).unwrap()
        );
    }

    #[test]
    fn test_get_month_end() {
        let date = NaiveDate::from_ymd_opt(2024, 6, 29).unwrap();
        assert_eq!(
            get_month_end(date),
            NaiveDate::from_ymd_opt(2024, 6, 30).unwrap()
        );
    }

    #[test]
    fn test_get_month_start_comprehensive() {
        let date = NaiveDate::from_ymd_opt(2024, 6, 29).unwrap();
        assert_eq!(
            get_month_start_comprehensive(date),
            NaiveDate::from_ymd_opt(2024, 5, 27).unwrap()
        );
    }

    #[test]
    fn test_get_month_end_comprehensive() {
        let date = NaiveDate::from_ymd_opt(2024, 6, 5).unwrap();
        assert_eq!(
            get_month_end_comprehensive(date),
            NaiveDate::from_ymd_opt(2024, 6, 30).unwrap()
        );
    }

    // #[test]
    // fn test_generate_week_range() {
    //     let date = NaiveDate::from_ymd_opt(2024, 6, 29).unwrap();
    //     let week_range = generate_week_range(date);
    //     let expected_dates: HashSet<String> = (24..=30)
    //         .map(|d| NaiveDate::from_ymd_opt(2024, 6, d).unwrap().to_string())
    //         .collect();
    //     assert_eq!(week_range, expected_dates);
    // }

    #[test]
    fn test_generate_month_range() {
        let date = NaiveDate::from_ymd_opt(2024, 6, 29).unwrap();
        let month_range = generate_month_range(date);
        let expected_dates: Vec<String> = (1..=30).map(|d| format!("{:02}", d)).collect();
        assert_eq!(month_range, expected_dates);
    }

    #[test]
    fn test_format_datetime() {
        let datetime = Utc::now();
        assert_eq!(
            format_datetime(&Some(datetime)),
            datetime.format(DATETIME_FORMAT_SHORT).to_string()
        );
        assert_eq!(format_datetime(&None), "-".to_string());
    }

    #[test]
    fn test_parse_date() {
        let date_str = "2024-06-29";
        let parsed_date = parse_date(date_str);
        assert_eq!(parsed_date, NaiveDate::from_ymd_opt(2024, 6, 29).unwrap());

        let invalid_date_str = "invalid-date";
        let fallback_date = Utc::now().date_naive();
        let parsed_date = parse_date(invalid_date_str);
        assert_eq!(parsed_date, fallback_date);
    }
}
