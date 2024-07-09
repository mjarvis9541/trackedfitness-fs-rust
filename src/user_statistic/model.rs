use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserStatistic {
    pub id: Uuid,
    pub username: String,
    pub profile_id: Option<Uuid>,
    pub follower_count: i64,
    pub following_count: i64,
    pub diet_count: i64,
    pub diet_day_log_count: i64,
    pub diet_target_count: i64,
    pub progress_count: i64,
    pub workout_count: i64,
    pub workout_day_log_count: i64,
    pub exercise_count: i64,
    pub set_count: i64,
    pub rep_count: i64,
    pub food_created_count: i64,
    pub brand_created_count: i64,
    pub meal_created_count: i64,
    pub meal_food_created_count: i64,
    pub meal_of_day_created_count: i64,
    pub movement_created_count: i64,
    pub muscle_group_created_count: i64,
}
