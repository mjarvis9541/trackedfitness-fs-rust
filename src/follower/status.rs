use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum FollowerStatus {
    Pending,
    Accepted,
    Declined,
    NotRequested,
    Unknown(i32),
}

impl fmt::Display for FollowerStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_str = match self {
            FollowerStatus::Pending => "Pending",
            FollowerStatus::Accepted => "Accepted",
            FollowerStatus::Declined => "Declined",
            FollowerStatus::NotRequested => "Not Requested",
            FollowerStatus::Unknown(value) => {
                return write!(f, "Unknown ({})", value);
            }
        };
        write!(f, "{}", display_str)
    }
}

impl From<FollowerStatus> for i32 {
    fn from(status: FollowerStatus) -> Self {
        match status {
            FollowerStatus::Pending => 0,
            FollowerStatus::Accepted => 1,
            FollowerStatus::Declined => 2,
            FollowerStatus::NotRequested => 3,
            FollowerStatus::Unknown(code) => code,
        }
    }
}

impl From<i32> for FollowerStatus {
    fn from(status: i32) -> Self {
        match status {
            0 => FollowerStatus::Pending,
            1 => FollowerStatus::Accepted,
            2 => FollowerStatus::Declined,
            3 => FollowerStatus::NotRequested,
            _ => FollowerStatus::Unknown(status),
        }
    }
}

impl FollowerStatus {
    pub fn can_request(&self) -> bool {
        match self {
            FollowerStatus::Pending => false,
            FollowerStatus::Accepted => false,
            FollowerStatus::Declined => false,
            FollowerStatus::NotRequested => true,
            FollowerStatus::Unknown(_) => true,
        }
    }

    pub fn can_unfollow(&self) -> bool {
        match self {
            FollowerStatus::Pending => true,
            FollowerStatus::Accepted => true,
            FollowerStatus::Declined => false,
            FollowerStatus::NotRequested => false,
            FollowerStatus::Unknown(_) => false,
        }
    }

    pub fn get_unfollow_wording(&self) -> &'static str {
        match self {
            FollowerStatus::Pending => "Cancel",
            FollowerStatus::Accepted => "Unfollow",
            _ => "",
        }
    }
}
