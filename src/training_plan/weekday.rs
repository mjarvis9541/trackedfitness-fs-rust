use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub enum WeekDay {
    #[default]
    NotApplicable,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl fmt::Display for WeekDay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WeekDay::NotApplicable => "N/A",
                WeekDay::Monday => "Monday",
                WeekDay::Tuesday => "Tuesday",
                WeekDay::Wednesday => "Wednesday",
                WeekDay::Thursday => "Thursday",
                WeekDay::Friday => "Friday",
                WeekDay::Saturday => "Saturday",
                WeekDay::Sunday => "Sunday",
            }
        )
    }
}

impl From<i32> for WeekDay {
    fn from(item: i32) -> Self {
        match item {
            1 => WeekDay::Monday,
            2 => WeekDay::Tuesday,
            3 => WeekDay::Wednesday,
            4 => WeekDay::Thursday,
            5 => WeekDay::Friday,
            6 => WeekDay::Saturday,
            7 => WeekDay::Sunday,
            _ => WeekDay::NotApplicable,
        }
    }
}

impl WeekDay {
    pub fn to_i32(&self) -> i32 {
        match self {
            WeekDay::Monday => 1,
            WeekDay::Tuesday => 2,
            WeekDay::Wednesday => 3,
            WeekDay::Thursday => 4,
            WeekDay::Friday => 5,
            WeekDay::Saturday => 6,
            WeekDay::Sunday => 7,
            WeekDay::NotApplicable => 0,
        }
    }
}
