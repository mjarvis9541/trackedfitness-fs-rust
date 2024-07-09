use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "ssr")]
#[allow(dead_code)]
#[derive(Debug)]
pub struct MuscleGroup {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by_id: Uuid,
    pub updated_by_id: Option<Uuid>,
}

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MuscleGroupQuery {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by_id: Uuid,
    pub updated_by_id: Option<Uuid>,
    pub exercise_count: i64,
    pub created_by: String,
    pub updated_by: Option<String>,
}
