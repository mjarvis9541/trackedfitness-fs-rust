use sqlx::postgres::PgRow;
use sqlx::Row;

use super::model::{Nutrition, NutritionPerKg};

impl Nutrition {
    pub fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        Ok(Self {
            energy: row.try_get("energy")?,
            fat: row.try_get("fat")?,
            saturates: row.try_get("saturates")?,
            carbohydrate: row.try_get("carbohydrate")?,
            sugars: row.try_get("sugars")?,
            fibre: row.try_get("fibre")?,
            protein: row.try_get("protein")?,
            salt: row.try_get("salt")?,
            protein_pct: row.try_get("protein_pct")?,
            carbohydrate_pct: row.try_get("carbohydrate_pct")?,
            fat_pct: row.try_get("fat_pct")?,
        })
    }

    pub fn from_row_with_prefix(row: &PgRow, prefix: &str) -> sqlx::Result<Self> {
        let field = |name: &str| -> String { format!("{}_{}", prefix, name) };
        Ok(Self {
            energy: row.try_get(&field("energy")[..])?,
            fat: row.try_get(&field("fat")[..])?,
            saturates: row.try_get(&field("saturates")[..])?,
            carbohydrate: row.try_get(&field("carbohydrate")[..])?,
            sugars: row.try_get(&field("sugars")[..])?,
            fibre: row.try_get(&field("fibre")[..])?,
            protein: row.try_get(&field("protein")[..])?,
            salt: row.try_get(&field("salt")[..])?,
            protein_pct: row.try_get(&field("protein_pct")[..])?,
            carbohydrate_pct: row.try_get(&field("carbohydrate_pct")[..])?,
            fat_pct: row.try_get(&field("fat_pct")[..])?,
        })
    }

    pub fn calculate_remaining(total: &Nutrition, target: &Nutrition) -> Nutrition {
        Nutrition {
            energy: target.energy - total.energy,
            fat: target.fat - total.fat,
            saturates: target.saturates - total.saturates,
            carbohydrate: target.carbohydrate - total.carbohydrate,
            sugars: target.sugars - total.sugars,
            fibre: target.fibre - total.fibre,
            protein: target.protein - total.protein,
            salt: target.salt - total.salt,
            ..Default::default()
        }
    }
}

impl NutritionPerKg {
    pub fn from_day_row(row: &PgRow) -> sqlx::Result<Self> {
        Ok(Self {
            energy_per_kg: row.try_get("day_energy_per_kg")?,
            protein_per_kg: row.try_get("day_protein_per_kg")?,
            carbohydrate_per_kg: row.try_get("day_carbohydrate_per_kg")?,
            fat_per_kg: row.try_get("day_fat_per_kg")?,
        })
    }

    pub fn from_target_row(row: &PgRow) -> sqlx::Result<Self> {
        Ok(Self {
            energy_per_kg: row.try_get("target_energy_per_kg")?,
            protein_per_kg: row.try_get("target_protein_per_kg")?,
            carbohydrate_per_kg: row.try_get("target_carbohydrate_per_kg")?,
            fat_per_kg: row.try_get("target_fat_per_kg")?,
        })
    }
}
