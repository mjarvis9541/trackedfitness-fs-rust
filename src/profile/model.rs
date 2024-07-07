use chrono::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

use super::activity_level::ActivityLevel;
use super::fitness_goal::FitnessGoal;
use super::sex::Sex;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProfileBase {
    pub id: Uuid,
    pub user_id: Uuid,
    pub sex: String,
    pub height: Decimal,
    pub date_of_birth: NaiveDate,
    pub fitness_goal: String,
    pub activity_level: String,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub created_by_id: Uuid,
    pub updated_by_id: Option<Uuid>,
    pub image_location: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProfileImage {
    pub user_id: Uuid,
    pub image_location: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Profile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub sex: String,
    pub height: Decimal,
    pub date_of_birth: NaiveDate,
    pub activity_level: String,
    pub fitness_goal: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by_id: Uuid,
    pub updated_by_id: Option<Uuid>,
    pub image_location: Option<String>,
    //
    pub latest_weight: Option<Decimal>,
    pub latest_weight_date: Option<NaiveDate>,
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProfileMetric {
    pub id: Uuid,
    pub user_id: Uuid,
    pub activity_level: String,
    pub sex: String,
    pub height: Decimal,
    // pub date_of_birth: NaiveDate,
    pub fitness_goal: String,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub created_by_id: Uuid,
    pub updated_by_id: Option<Uuid>,
    pub image_location: Option<String>,
    //
    pub latest_weight: Option<Decimal>,
    pub latest_weight_date: Option<NaiveDate>,
    pub username: String,
    //
    pub age: u32,
    pub activity_level_display: String,
    pub basal_metabolic_rate: Decimal,
    pub body_mass_index: Decimal,
    pub fitness_goal_display: String,
    pub sex_display: String,
    pub target_calories: Decimal,
    pub total_daily_energy_expenditure: Decimal,
}

impl From<Profile> for ProfileMetric {
    fn from(data: Profile) -> Self {
        // let fitness_goal = data.fitness_goal.parse::<FitnessGoal>().unwrap_or_default();
        // let activity_level = data
        //     .activity_level
        //     .parse::<ActivityLevel>()
        //     .unwrap_or_default();
        // let sex = data.sex.parse::<Sex>().unwrap_or_default();
        let age = data.get_age();

        let activity_level_display = ActivityLevel::from_str(&data.activity_level)
            .unwrap_or_default()
            .to_string();
        let fitness_goal_display = FitnessGoal::from_str(&data.fitness_goal)
            .unwrap_or_default()
            .to_string();
        let sex_display = Sex::from_str(&data.sex).unwrap_or_default().to_string();

        let basal_metabolic_rate = data.get_basal_metabolic_rate();
        let body_mass_index = data.get_body_mass_index();
        let total_daily_energy_expenditure = data.get_total_daily_energy_expenditure();
        let target_calories = data.get_target_calories();

        ProfileMetric {
            id: data.id,
            user_id: data.user_id,
            activity_level: data.activity_level,
            sex: data.sex,
            height: data.height,
            age,
            fitness_goal: data.fitness_goal,
            updated_at: data.updated_at,
            created_at: data.created_at,
            created_by_id: data.created_by_id,
            updated_by_id: data.updated_by_id,
            image_location: data.image_location,
            latest_weight: data.latest_weight,
            latest_weight_date: data.latest_weight_date,
            username: data.username,
            activity_level_display,
            basal_metabolic_rate,
            body_mass_index,
            fitness_goal_display,
            sex_display,
            target_calories,
            total_daily_energy_expenditure,
        }
    }
}

impl Profile {
    pub fn get_age(&self) -> u32 {
        Utc::now()
            .date_naive()
            .years_since(self.date_of_birth)
            .unwrap_or(0)
    }

    /// Body mass index, loosely defines a given persons health mass based of a height to width ratio.
    pub fn get_body_mass_index(&self) -> Decimal {
        let Some(latest_weight) = self.latest_weight else {
            return Decimal::from(0);
        };
        let height_in_meters = self.height * Decimal::new(1, 2);
        let body_mass_index = latest_weight / (height_in_meters * height_in_meters);
        body_mass_index
    }

    /// Basal metabolic rate, calculated using the Harris-Benedict Equation
    pub fn get_basal_metabolic_rate(&self) -> Decimal {
        let Some(weight) = self.latest_weight else {
            return Decimal::from(0);
        };
        let sex = Sex::from_str(&self.sex).unwrap_or(Sex::Default);
        let bmr_modifier = sex.to_bmr_modifier();
        let age_decimal = Decimal::from(self.get_age());
        let height = self.height;
        let basal_metabolic_rate = bmr_modifier.sex_modifier
            + (weight * bmr_modifier.weight_modifier)
            + (height * bmr_modifier.height_modifier)
            - (bmr_modifier.age_modifier * age_decimal);
        basal_metabolic_rate
    }

    /// Estimation of daily energy burnt taking into account life-sustaining functions and physical activity;
    /// A pretty accurate estimataion.
    pub fn get_total_daily_energy_expenditure(&self) -> Decimal {
        let activity_level =
            ActivityLevel::from_str(&self.activity_level).unwrap_or(ActivityLevel::Sedentary);
        let activity_level_modifier = activity_level.to_tdee_modifier();
        let basal_metabolic_rate = self.get_basal_metabolic_rate();
        let total_daily_energy_expenditure = basal_metabolic_rate * activity_level_modifier;
        total_daily_energy_expenditure
    }

    /// Add a 10% surplus / 25% deficiet to target calories based on goals and time accomodation for exercise.
    pub fn get_target_calories(&self) -> Decimal {
        let fitness_goal =
            FitnessGoal::from_str(&self.fitness_goal).unwrap_or(FitnessGoal::MaintainWeight);
        let fitness_goal_modifier = fitness_goal.to_calorie_modifier();
        let total_daily_energy_expenditure = self.get_total_daily_energy_expenditure();
        let target_calories = total_daily_energy_expenditure * fitness_goal_modifier;
        target_calories
    }
}
