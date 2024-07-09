use chrono::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "ssr")]
#[allow(dead_code)]
#[derive(Debug)]
pub struct Progress {
    pub id: Uuid,
    pub user_id: Uuid,
    pub date: NaiveDate,
    pub weight_kg: Option<Decimal>,
    pub energy_burnt: Option<i32>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by_id: Uuid,
    pub updated_by_id: Option<Uuid>,
}

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProgressQuery {
    pub id: Uuid,
    pub user_id: Uuid,
    pub date: NaiveDate,
    pub weight: Option<Decimal>,
    pub week_avg_weight: Option<Decimal>,
    pub month_avg_weight: Option<Decimal>,
    pub energy_burnt: Option<i32>,
    pub week_avg_energy_burnt: Option<i32>,
    pub month_avg_energy_burnt: Option<i32>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub username: String,
}
