use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum PrivacyLevel {
    Public,
    Followers,
    Private,
    Unknown(i32),
}

impl fmt::Display for PrivacyLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrivacyLevel::Public => write!(f, "Public"),
            PrivacyLevel::Followers => write!(f, "Followers"),
            PrivacyLevel::Private => write!(f, "Private"),
            PrivacyLevel::Unknown(value) => write!(f, "Unknown ({})", value),
        }
    }
}

impl PrivacyLevel {
    pub fn to_form_options() -> Vec<(&'static str, &'static str)> {
        let options = [
            ("0", "N/A - All users can view your profile"),
            ("1", "Public - All users can view your profile"),
            ("2", "Followers Only - Only followers can view your profile"),
            ("3", "Private - No users can view your profile"),
        ];
        options.to_vec()
    }

    pub fn to_form_value(&self) -> &'static str {
        match self {
            PrivacyLevel::Public => "1",
            PrivacyLevel::Followers => "2",
            PrivacyLevel::Private => "3",
            PrivacyLevel::Unknown(_) => "0",
        }
    }
}

impl From<i32> for PrivacyLevel {
    fn from(value: i32) -> Self {
        match value {
            1 => PrivacyLevel::Public,
            2 => PrivacyLevel::Followers,
            3 => PrivacyLevel::Private,
            _ => PrivacyLevel::Unknown(value),
        }
    }
}

impl From<PrivacyLevel> for i32 {
    fn from(level: PrivacyLevel) -> Self {
        match level {
            PrivacyLevel::Public => 1,
            PrivacyLevel::Followers => 2,
            PrivacyLevel::Private => 3,
            PrivacyLevel::Unknown(value) => value,
        }
    }
}
