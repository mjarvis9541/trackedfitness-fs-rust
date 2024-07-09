use chrono::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct DietTarget {
    pub id: Uuid,
    pub user_id: Uuid,
    pub date: NaiveDate,
    pub weight: Decimal,
    pub energy: i32,
    pub fat: Decimal,
    pub saturates: Decimal,
    pub carbohydrate: Decimal,
    pub sugars: Decimal,
    pub fibre: Decimal,
    pub protein: Decimal,
    pub salt: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by_id: Uuid,
    pub updated_by_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DietTargetQuery {
    pub id: Uuid,
    pub user_id: Uuid,
    pub date: NaiveDate,
    pub weight: Decimal,
    pub energy: i32,
    pub fat: Decimal,
    pub saturates: Decimal,
    pub carbohydrate: Decimal,
    pub sugars: Decimal,
    pub fibre: Decimal,
    pub protein: Decimal,
    pub salt: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by_id: Uuid,
    pub updated_by_id: Option<Uuid>,
    pub protein_pct: Decimal,
    pub carbohydrate_pct: Decimal,
    pub fat_pct: Decimal,
    pub energy_per_kg: Decimal,
    pub protein_per_kg: Decimal,
    pub carbohydrate_per_kg: Decimal,
    pub fat_per_kg: Decimal,
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DietTargetGramKg {
    pub user_id: Uuid,
    pub date: NaiveDate,
    pub weight: Decimal,
    pub protein_per_kg: Decimal,
    pub carbohydrate_per_kg: Decimal,
    pub fat_per_kg: Decimal,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DietTargetInput {
    pub user_id: Uuid,
    pub date: NaiveDate,
    pub weight: Decimal,
    pub energy: i32,
    pub fat: Decimal,
    pub saturates: Decimal,
    pub carbohydrate: Decimal,
    pub sugars: Decimal,
    pub fibre: Decimal,
    pub protein: Decimal,
    pub salt: Decimal,
}
