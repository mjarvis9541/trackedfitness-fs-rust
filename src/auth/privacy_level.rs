use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub enum PrivacyLevel {
    Public,
    #[default]
    Followers,
    Private,
    Unknown(i32),
}

impl fmt::Display for PrivacyLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_str = match self {
            PrivacyLevel::Public => "Public",
            PrivacyLevel::Followers => "Followers",
            PrivacyLevel::Private => "Private",
            PrivacyLevel::Unknown(value) => return write!(f, "Unknown ({})", value),
        };
        write!(f, "{}", display_str)
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
