use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "ssr")]
#[allow(dead_code)]
#[derive(Debug)]
pub struct Brand {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub image_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by_id: Uuid,
    pub updated_by_id: Option<Uuid>,
}

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BrandQuery {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub image_url: Option<String>,
    pub food_count: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by_id: Uuid,
    pub updated_by_id: Option<Uuid>,
    pub created_by: String,
    pub updated_by: Option<String>,
}
