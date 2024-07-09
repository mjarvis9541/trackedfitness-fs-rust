use std::collections::BTreeMap;

use chrono::prelude::*;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::diet_target::model::DietTargetQuery;
use crate::error::Result;
use crate::food::model::Nutrition;
use crate::meal_of_day::model::MealOfDay;

use super::day_page::DietDayResponse;
use super::model::{DietDayDTO, DietFoodQuery, DietMealDTO};

// impl From<DietFoodQuery> for DietFoodQuery {
//     fn from(data: DietFoodQuery) -> Self {
//         DietFoodQuery { ..data.into() }
//     }
// }

impl DietFoodQuery {
    pub fn group_by_meal_of_day_id(
        foods: Vec<DietFoodQuery>,
    ) -> BTreeMap<Uuid, Vec<DietFoodQuery>> {
        let mut grouped = BTreeMap::new();
        for food in foods {
            grouped
                .entry(food.meal_of_day_id)
                .or_insert_with(Vec::new)
                .push(food);
        }
        grouped
    }
}

impl DietMealDTO {
    fn calculate_percentages(&mut self) {
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
}

impl DietDayDTO {
    fn calculate_percentages(&mut self) {
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
}

pub struct DietService;

impl DietService {
    pub fn group_diet_food_by_meal_of_day(
        diet_food: Vec<DietFoodQuery>,
    ) -> BTreeMap<Uuid, Vec<DietFoodQuery>> {
        let mut grouped: BTreeMap<Uuid, Vec<DietFoodQuery>> = BTreeMap::new();
        for food in diet_food {
            grouped
                .entry(food.meal_of_day_id)
                .or_insert_with(Vec::new)
                .push(food);
        }
        grouped
    }

    pub async fn aggregate_diet_day_data(
        pool: &PgPool,
        username: &str,
        date: NaiveDate,
    ) -> Result<DietDayResponse> {
        let meal_of_day = MealOfDay::all(pool).await?;
        let diet_target =
            DietTargetQuery::get_latest_by_username_date(pool, username, date).await?;
        let diet_food = DietFoodQuery::all_by_username_date(pool, username, date).await?;

        let mut diet_day = DietDayDTO {
            username: username.to_string(),
            date,
            ..Default::default()
        };

        let mut grouped_diet_food = Self::group_diet_food_by_meal_of_day(diet_food);
        // let mut grouped_diet_food: BTreeMap<Uuid, Vec<DietFoodQuery>> = BTreeMap::new();
        // for food in diet_food {
        //     grouped_diet_food
        //         .entry(food.meal_of_day_id)
        //         .or_insert_with(Vec::new)
        //         .push(food);
        // }
        let meal_dto_list: Vec<DietMealDTO> = meal_of_day
            .into_iter()
            .map(|meal| {
                let mut meal_dto = DietMealDTO {
                    id: meal.id,
                    date,
                    username: username.to_string(),
                    name: meal.name,
                    slug: meal.slug,
                    ordering: meal.ordering,
                    ..Default::default()
                };
                if let Some(food_list) = grouped_diet_food.get(&meal.id) {
                    for food in food_list {
                        meal_dto.username = food.username.clone();
                        meal_dto.date = food.date;
                        meal_dto.energy += food.energy;
                        meal_dto.fat += food.fat;
                        meal_dto.saturates += food.saturates;
                        meal_dto.carbohydrate += food.carbohydrate;
                        meal_dto.sugars += food.sugars;
                        meal_dto.fibre += food.fibre;
                        meal_dto.protein += food.protein;
                        meal_dto.salt += food.salt;
                    }
                    meal_dto.food_list = grouped_diet_food.remove(&meal.id).unwrap_or_default();
                    meal_dto.calculate_percentages();
                }
                meal_dto
            })
            .collect();

        for meal in &meal_dto_list {
            diet_day.energy += meal.energy;
            diet_day.fat += meal.fat;
            diet_day.saturates += meal.saturates;
            diet_day.carbohydrate += meal.carbohydrate;
            diet_day.sugars += meal.sugars;
            diet_day.fibre += meal.fibre;
            diet_day.protein += meal.protein;
            diet_day.salt += meal.salt;
        }

        diet_day.meal_list = meal_dto_list;
        diet_day.calculate_percentages();

        let remaining = if let Some(target) = &diet_target {
            let energy = Decimal::from(target.energy);
            let mut nut = Nutrition {
                energy: energy - diet_day.energy,
                fat: target.fat - diet_day.fat,
                saturates: target.saturates - diet_day.saturates,
                carbohydrate: target.carbohydrate - diet_day.carbohydrate,
                sugars: target.sugars - diet_day.sugars,
                fibre: target.fibre - diet_day.fibre,
                protein: target.protein - diet_day.protein,
                salt: target.salt - diet_day.salt,
                ..Default::default()
            };
            nut.calculate_percentages();
            Some(nut)
        } else {
            None
        };

        let response = DietDayResponse {
            diet_day,
            diet_target,
            remaining,
        };

        Ok(response)
    }
}
