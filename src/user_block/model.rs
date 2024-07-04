use std::fmt;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserBlock {
    pub id: Uuid,
    pub blocker_id: Uuid,
    pub blocked_id: Uuid,
    pub blocked_status: UserBlockStatus,
    pub blocked_at: DateTime<Utc>,
    pub unblocked_at: Option<DateTime<Utc>>,
    pub blocker_username: String,
    pub blocked_username: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum UserBlockStatus {
    Unblocked,
    Blocked,
    Unknown(i32),
}

impl fmt::Display for UserBlockStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_str = match self {
            UserBlockStatus::Unblocked => "Unblocked",
            UserBlockStatus::Blocked => "Blocked",
            UserBlockStatus::Unknown(value) => {
                return write!(f, "Unknown ({})", value);
            }
        };
        write!(f, "{}", display_str)
    }
}

impl From<i32> for UserBlockStatus {
    fn from(status: i32) -> Self {
        match status {
            0 => UserBlockStatus::Unblocked,
            1 => UserBlockStatus::Blocked,
            _ => UserBlockStatus::Unknown(status),
        }
    }
}
