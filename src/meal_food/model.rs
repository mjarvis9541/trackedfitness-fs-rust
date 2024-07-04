use chrono::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::food::model::Nutrition;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MealFoodModel {
    // pub id: Uuid,
    // pub meal_id: Uuid,
    pub food_id: Uuid,
    pub quantity: Decimal,
    // pub created_at: DateTime<Utc>,
    // pub updated_at: Option<DateTime<Utc>>,
    // pub created_by_id: Uuid,
    // pub updated_by_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MealFood {
    pub id: Uuid,
    pub meal_id: Uuid,
    pub food_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by_id: Uuid,
    pub updated_by_id: Option<Uuid>,
    pub food_name: String,
    pub food_slug: String,
    pub brand_name: String,
    pub brand_slug: String,
    pub data_value: Decimal,
    pub data_measurement: String,
    pub nutrition: Nutrition,
}
