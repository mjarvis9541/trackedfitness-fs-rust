use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::food::model::Nutrition;

#[derive(Debug, Deserialize, Serialize)]
pub struct Meal {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by_id: Uuid,
    pub updated_by_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct MealQuery {
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

impl MealQuery {
    const SORT_OPTIONS_DISPLAY: &'static [(&'static str, &'static str)] = &[
        ("name", "Name (A-z)"),
        ("-name", "Name (Z-a)"),
        ("-food_count", "Food Count (High-Low)"),
        ("food_count", "Food Count (Low-High)"),
        ("-energy", "Calories (High-Low)"),
        ("energy", "Calories (Low-High)"),
        ("-protein", "Protein (High-Low)"),
        ("protein", "Protein (Low-High)"),
        ("-carbohydrate", "Carbs (High-Low)"),
        ("carbohydrate", "Carbs (Low-High)"),
        ("-fat", "Fat (High-Low)"),
        ("fat", "Fat (Low-High)"),
        ("-saturates", "Saturates (High-Low)"),
        ("saturates", "Saturates (Low-High)"),
        ("-sugars", "Sugars (High-Low)"),
        ("sugars", "Sugars (Low-High)"),
        ("-fibre", "Fibre (High-Low)"),
        ("fibre", "Fibre (Low-High)"),
        ("-salt", "Salt (High-Low)"),
        ("salt", "Salt (Low-High)"),
        ("-created_at", "Created (Desc)"),
        ("created_at", "Created (Asc)"),
        ("-updated_at", "Updated (Desc)"),
        ("updated_at", "Updated (Asc)"),
    ];

    pub fn get_detail_href(&self) -> String {
        format!("/food/meals/{}", self.id)
    }

    pub fn to_sort_options() -> Vec<(&'static str, &'static str)> {
        let options = Self::SORT_OPTIONS_DISPLAY;
        options.to_vec()
    }
}
