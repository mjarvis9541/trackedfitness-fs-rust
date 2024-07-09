use serde::{Deserialize, Serialize};

use crate::auth::privacy_level::PrivacyLevel;
use crate::follower::status::FollowerStatus;
use crate::user_block::model::UserBlockStatus;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserQuery {
    pub name: String,
    pub username: String,
    pub is_self: bool,
    pub can_view: bool,
    pub privacy_level: PrivacyLevel,
    pub blocked_status: UserBlockStatus,
    pub follower_status: FollowerStatus,
    pub follower_count: i64,
    pub following_count: i64,
}
