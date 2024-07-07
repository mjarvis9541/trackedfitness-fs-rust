use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::food::model::Nutrition;

#[derive(Debug, Deserialize, Serialize)]
pub struct MealBase {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by_id: Uuid,
    pub updated_by_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Meal {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by_id: Uuid,
    pub updated_by_id: Option<Uuid>,
    pub food_count: i64,
    pub username: String,
    pub nutrition: Nutrition,
}

impl Meal {
    pub fn get_detail_href(&self) -> String {
        format!("/food/meals/{}", self.id)
    }
}
