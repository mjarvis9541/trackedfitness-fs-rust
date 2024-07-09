use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MealOfDay {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub ordering: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by_id: Uuid,
    pub updated_by_id: Option<Uuid>,
}
