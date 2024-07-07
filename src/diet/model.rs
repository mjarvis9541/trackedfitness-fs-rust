use std::collections::HashSet;

use chrono::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::diet_target::model::DietTarget;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Diet {
    pub id: Uuid,
    pub date: NaiveDate,
    pub user_id: Uuid,
    pub food_id: Uuid,
    pub meal_of_day_id: Uuid,
    pub quantity: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by_id: Uuid,
    pub updated_by_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DietWeekNav {
    pub username: String,
    pub date: NaiveDate,
    pub energy: Decimal,
    pub protein: Decimal,
    pub carbohydrate: Decimal,
    pub fat: Decimal,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DietFoodQuery {
    pub id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub date: NaiveDate,
    pub meal_of_day_id: Uuid,
    pub meal_of_day_name: String,
    pub meal_of_day_slug: String,
    pub meal_of_day_ordering: i32,
    pub food_id: Uuid,
    pub food_name: String,
    pub food_slug: String,
    pub brand_id: Uuid,
    pub brand_name: String,
    pub brand_slug: String,
    pub data_value: Decimal,
    pub data_measurement: String,
    pub energy: Decimal,
    pub fat: Decimal,
    pub saturates: Decimal,
    pub carbohydrate: Decimal,
    pub sugars: Decimal,
    pub fibre: Decimal,
    pub protein: Decimal,
    pub salt: Decimal,
    pub protein_pct: Decimal,
    pub carbohydrate_pct: Decimal,
    pub fat_pct: Decimal,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DietMealQuery {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub ordering: i32,
    pub user_id: Uuid,
    pub energy: Decimal,
    pub fat: Decimal,
    pub saturates: Decimal,
    pub carbohydrate: Decimal,
    pub sugars: Decimal,
    pub fibre: Decimal,
    pub protein: Decimal,
    pub salt: Decimal,
    pub protein_pct: Decimal,
    pub carbohydrate_pct: Decimal,
    pub fat_pct: Decimal,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct DietDayQuery {
    pub user_id: Uuid,
    pub date: NaiveDate,
    pub energy: Decimal,
    pub fat: Decimal,
    pub saturates: Decimal,
    pub carbohydrate: Decimal,
    pub sugars: Decimal,
    pub fibre: Decimal,
    pub protein: Decimal,
    pub salt: Decimal,
    pub protein_pct: Decimal,
    pub carbohydrate_pct: Decimal,
    pub fat_pct: Decimal,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct DietMealDTO {
    pub id: Uuid,
    pub username: String,
    pub date: NaiveDate,
    pub name: String,
    pub slug: String,
    pub ordering: i32,
    pub energy: Decimal,
    pub fat: Decimal,
    pub saturates: Decimal,
    pub carbohydrate: Decimal,
    pub sugars: Decimal,
    pub fibre: Decimal,
    pub protein: Decimal,
    pub salt: Decimal,
    pub protein_pct: Decimal,
    pub carbohydrate_pct: Decimal,
    pub fat_pct: Decimal,
    pub food_list: Vec<DietFoodQuery>,
}

impl DietMealDTO {
    pub fn meal_detail_url(&self) -> String {
        format!("/users/{}/diet/{}/{}", self.username, self.date, self.slug)
    }

    pub fn diet_add_food_url(&self) -> String {
        format!(
            "/users/{}/diet/{}/{}/add-food",
            self.username, self.date, self.slug
        )
    }

    pub fn diet_add_meal_url(&self) -> String {
        format!(
            "/users/{}/diet/{}/{}/add-meal",
            self.username, self.date, self.slug
        )
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct DietDayDTO {
    pub user_id: Uuid,
    pub username: String,
    pub date: NaiveDate,
    pub energy: Decimal,
    pub fat: Decimal,
    pub saturates: Decimal,
    pub carbohydrate: Decimal,
    pub sugars: Decimal,
    pub fibre: Decimal,
    pub protein: Decimal,
    pub salt: Decimal,
    pub protein_pct: Decimal,
    pub carbohydrate_pct: Decimal,
    pub fat_pct: Decimal,
    pub meal_list: Vec<DietMealDTO>,
}

impl DietFoodQuery {
    pub fn title(&self) -> String {
        if self.data_measurement == "srv" {
            format!(
                "{}, {:1}{}",
                self.food_name, self.data_value, self.data_measurement
            )
        } else {
            format!(
                "{}, {:0}{}",
                self.food_name, self.data_value, self.data_measurement
            )
        }
    }
    pub fn get_serving_display(&self) -> String {
        if self.data_measurement == "srv" {
            format!("{:.1}{}", self.data_value, self.data_measurement)
        } else {
            format!("{:.0}{}", self.data_value, self.data_measurement)
        }
    }

    pub fn food_detail_url(&self) -> String {
        format!("/food/{}", self.food_slug)
    }

    pub fn brand_detail_url(&self) -> String {
        format!("/food/brands/{}", self.brand_slug)
    }

    pub fn diet_day_url(&self) -> String {
        format!("/users/{}/diet/{}", self.username, self.date)
    }

    pub fn diet_detail_url(&self) -> String {
        format!(
            "/users/{}/diet/{}/{}/{}",
            self.username, self.date, self.meal_of_day_slug, self.id
        )
    }

    pub fn diet_update_url(&self) -> String {
        format!(
            "/users/{}/diet/{}/{}/{}/update",
            self.username, self.date, self.meal_of_day_slug, self.id
        )
    }

    pub fn diet_delete_url(&self) -> String {
        format!(
            "/users/{}/diet/{}/{}/{}/delete",
            self.username, self.date, self.meal_of_day_slug, self.id
        )
    }

    pub fn ids_as_set(food_list: &[DietFoodQuery]) -> HashSet<String> {
        food_list
            .iter()
            .map(|diet_food| diet_food.id.to_string())
            .collect()
    }
}

#[derive(Debug)]
pub struct FormattedFoodData {
    pub energy: String,
    pub fat: String,
    pub saturates: String,
    pub carbohydrate: String,
    pub sugars: String,
    pub fibre: String,
    pub protein: String,
    pub salt: String,
    pub protein_pct: String,
    pub carbohydrate_pct: String,
    pub fat_pct: String,
}

impl DietFoodQuery {
    pub fn format(&self) -> FormattedFoodData {
        FormattedFoodData {
            energy: format!("{:.0}kcal", self.energy),
            fat: format!("{:.1}g", self.fat),
            saturates: format!("{:.1}g", self.saturates),
            carbohydrate: format!("{:.1}g", self.carbohydrate),
            sugars: format!("{:.1}g", self.sugars),
            fibre: format!("{:.1}g", self.fibre),
            protein: format!("{:.1}g", self.protein),
            salt: format!("{:.2}g", self.salt),
            protein_pct: format!("{:.1}%", self.protein_pct),
            carbohydrate_pct: format!("{:.1}%", self.carbohydrate_pct),
            fat_pct: format!("{:.1}%", self.fat_pct),
        }
    }
}

impl DietMealDTO {
    pub fn format(&self) -> FormattedFoodData {
        FormattedFoodData {
            energy: format!("{:.0}kcal", self.energy),
            fat: format!("{:.1}g", self.fat),
            saturates: format!("{:.1}g", self.saturates),
            carbohydrate: format!("{:.1}g", self.carbohydrate),
            sugars: format!("{:.1}g", self.sugars),
            fibre: format!("{:.1}g", self.fibre),
            protein: format!("{:.1}g", self.protein),
            salt: format!("{:.2}g", self.salt),
            protein_pct: format!("{:.1}%", self.protein_pct),
            carbohydrate_pct: format!("{:.1}%", self.carbohydrate_pct),
            fat_pct: format!("{:.1}%", self.fat_pct),
        }
    }
}

impl DietDayDTO {
    pub fn format(&self) -> FormattedFoodData {
        FormattedFoodData {
            energy: format!("{:.0}kcal", self.energy),
            fat: format!("{:.1}g", self.fat),
            saturates: format!("{:.1}g", self.saturates),
            carbohydrate: format!("{:.1}g", self.carbohydrate),
            sugars: format!("{:.1}g", self.sugars),
            fibre: format!("{:.1}g", self.fibre),
            protein: format!("{:.1}g", self.protein),
            salt: format!("{:.2}g", self.salt),
            protein_pct: format!("{:.1}%", self.protein_pct),
            carbohydrate_pct: format!("{:.1}%", self.carbohydrate_pct),
            fat_pct: format!("{:.1}%", self.fat_pct),
        }
    }
}

impl DietTarget {
    pub fn format(&self) -> FormattedFoodData {
        FormattedFoodData {
            energy: format!("{:.0}kcal", self.energy),
            fat: format!("{:.1}g", self.fat),
            saturates: format!("{:.1}g", self.saturates),
            carbohydrate: format!("{:.1}g", self.carbohydrate),
            sugars: format!("{:.1}g", self.sugars),
            fibre: format!("{:.1}g", self.fibre),
            protein: format!("{:.1}g", self.protein),
            salt: format!("{:.2}g", self.salt),
            protein_pct: format!("{:.1}%", self.protein_pct),
            carbohydrate_pct: format!("{:.1}%", self.carbohydrate_pct),
            fat_pct: format!("{:.1}%", self.fat_pct),
        }
    }
}
