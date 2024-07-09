use chrono::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::diet::model::FormattedFoodData;

use super::data_measurement::DataMeasurement;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Food {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub brand_id: Uuid,
    pub data_value: i32,
    pub data_measurement: String,
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
    pub food_code: String,
    pub food_description: Option<String>,
    pub food_category: Option<String>,
    pub food_data_source: String,
    pub data_value_numeric: Decimal,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FoodQuery {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub brand_id: Uuid,
    pub data_value: i32,
    pub data_measurement: DataMeasurement,
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
    pub brand_name: String,
    pub brand_slug: String,
    pub brand_image_url: Option<String>,
    pub food_code: String,
    pub food_description: Option<String>,
    pub food_category: Option<String>,
    pub food_data_source: String,
    pub data_value_numeric: Decimal,
    pub last_added_quantity: Option<Decimal>,
    pub last_added_date: Option<NaiveDate>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Nutrition {
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

impl Nutrition {
    pub fn calculate_percentages(&mut self) {
        let protein_energy = self.protein * Decimal::new(4, 0);
        let carbohydrate_energy = self.carbohydrate * Decimal::new(4, 0);
        let fat_energy = self.fat * Decimal::new(9, 0);
        let total_energy = protein_energy + carbohydrate_energy + fat_energy;

        if total_energy > Decimal::new(0, 0) {
            self.protein_pct = (protein_energy / total_energy) * Decimal::new(100, 0);
            self.carbohydrate_pct = (carbohydrate_energy / total_energy) * Decimal::new(100, 0);
            self.fat_pct = (fat_energy / total_energy) * Decimal::new(100, 0);
        }
    }

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
            protein_pct: format!("{:.0}%", self.protein_pct),
            carbohydrate_pct: format!("{:.0}%", self.carbohydrate_pct),
            fat_pct: format!("{:.0}%", self.fat_pct),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct NutritionPerKg {
    pub energy_per_kg: Decimal,
    pub protein_per_kg: Decimal,
    pub carbohydrate_per_kg: Decimal,
    pub fat_per_kg: Decimal,
}

impl FoodQuery {
    pub fn get_title(&self) -> String {
        format!(
            "{}, {:.0}{}",
            self.name, self.data_value, self.data_measurement
        )
    }

    pub fn get_value_display(&self) -> String {
        match self.data_measurement {
            DataMeasurement::Servings => format!("Per {} Serving", self.data_value),
            _ => format!("Per {}{}", self.data_value, self.data_measurement),
        }
    }

    pub fn get_brand_url(&self) -> String {
        format!("/food/brands/{}", self.brand_slug)
    }

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
            protein_pct: format!("{:.2}%", self.protein_pct),
            carbohydrate_pct: format!("{:.2}%", self.carbohydrate_pct),
            fat_pct: format!("{:.2}%", self.fat_pct),
        }
    }

    pub fn get_last_added_data_value(&self) -> Decimal {
        self.last_added_quantity.map_or_else(
            || Decimal::from(self.data_value),
            |last_added_quantity| match self.data_measurement {
                DataMeasurement::Servings => last_added_quantity,
                _ => last_added_quantity * Decimal::from(100),
            },
        )
        // if let Some(last) = self.last_added_quantity {
        //     last * self.data_measurement.get_quantity_value()
        // } else {
        //     Decimal::from(self.data_value)
        // }
    }

    const SORT_DISPLAY: &'static [(&'static str, &'static str)] = &[
        ("name", "Food (A-z)"),
        ("-name", "Food (Z-a)"),
        ("brand_name", "Brand (A-z)"),
        ("-brand_name", "Brand (Z-a)"),
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

    const SORT_DISPLAY_DIET: &'static [(&'static str, &'static str)] = &[
        ("-last_added_quantity", "last_added_quantity (High-Low)"),
        ("last_added_quantity", "last_added_quantity (Low-High)"),
        ("-last_added_date", "last_added_date (High-Low)"),
        ("last_added_date", "last_added_date (Low-High)"),
    ];

    pub fn to_filter_options() -> Vec<(&'static str, &'static str)> {
        let options = Self::SORT_DISPLAY;
        options.to_vec()
    }

    pub fn to_diet_filter_options() -> Vec<(&'static str, &'static str)> {
        let combined = Self::SORT_DISPLAY
            .iter()
            .chain(Self::SORT_DISPLAY_DIET.iter())
            .copied()
            .collect();
        combined
    }
}
