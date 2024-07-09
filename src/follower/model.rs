use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::follower::status::FollowerStatus;

// #[derive(Debug)]
// pub struct FollowerBase {
//     pub id: Uuid,
//     pub user_id: Uuid,
//     pub follower_id: Uuid,
//     pub status: FollowerStatus,
//     pub created_at: DateTime<Utc>,
//     pub updated_at: Option<DateTime<Utc>>,
// }

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Follower {
    pub id: Uuid,
    pub user_id: Uuid,
    pub follower_id: Uuid,
    pub status: FollowerStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub username: String,
    pub follower: String,
}
