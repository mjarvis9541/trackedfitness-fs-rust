use sqlx::PgPool;

use crate::error::Result;
use crate::util::database::Filter;

use super::model::UserStatistic;
impl UserStatistic {
    pub async fn get_by_username(pool: &PgPool, username: &str) -> Result<Option<Self>> {
        let query = sqlx::query_file_as!(Self, "sql/user_statistic_by_username.sql", username)
            .fetch_optional(pool)
            .await?;
        Ok(query)
    }

    pub async fn filter(
        pool: &PgPool,
        search: &str,
        order: &str,
        size: i64,
        page: i64,
    ) -> Result<Vec<Self>> {
        let order_by_column = match order {
            "username" => "username",
            "-username" => "username DESC",
            "follower_count" => "follower_count",
            "-follower_count" => "follower_count DESC",
            "following_count" => "following_count",
            "-following_count" => "following_count DESC",
            "diet_count" => "diet_count",
            "-diet_count" => "diet_count DESC",
            "diet_day_log_count" => "diet_day_log_count",
            "-diet_day_log_count" => "diet_day_log_count DESC",
            "diet_target_count" => "diet_target_count",
            "-diet_target_count" => "diet_target_count DESC",
            "progress_count" => "progress_count",
            "-progress_count" => "progress_count DESC",
            "workout_count" => "workout_count",
            "-workout_count" => "workout_count DESC",
            "workout_day_log_count" => "workout_day_log_count",
            "-workout_day_log_count" => "workout_day_log_count DESC",
            "exercise_count" => "exercise_count",
            "-exercise_count" => "exercise_count DESC",
            "set_count" => "set_count",
            "-set_count" => "set_count DESC",
            "rep_count" => "rep_count",
            "-rep_count" => "rep_count DESC",
            "food_created_count" => "food_created_count",
            "-food_created_count" => "food_created_count DESC",
            "brand_created_count" => "brand_created_count",
            "-brand_created_count" => "brand_created_count DESC",
            "meal_created_count" => "meal_created_count",
            "-meal_created_count" => "meal_created_count DESC",
            "meal_food_created_count" => "meal_food_created_count",
            "-meal_food_created_count" => "meal_food_created_count DESC",
            "meal_of_day_created_count" => "meal_of_day_created_count",
            "-meal_of_day_created_count" => "meal_of_day_created_count DESC",
            "movement_created_count" => "movement_created_count",
            "-movement_created_count" => "movement_created_count DESC",
            "muscle_group_created_count" => "muscle_group_created_count",
            "-muscle_group_created_count" => "muscle_group_created_count DESC",
            _ => "username",
        };
        let mut qb = sqlx::QueryBuilder::new(
            "
            SELECT
                t1.id,
                t1.username,
                t2.id AS profile_id,
                COALESCE(food_count.food_created_count, 0) AS food_created_count,
                COALESCE(brand_count.brand_created_count, 0) AS brand_created_count,
                COALESCE(diet_count.diet_count, 0) AS diet_count,
                COALESCE(diet_day_count.diet_day_count, 0) AS diet_day_log_count,
                COALESCE(diet_target_count.diet_target_count, 0) AS diet_target_count,
                COALESCE(progress_count.progress_count, 0) AS progress_count,
                COALESCE(meal_count.meal_created_count, 0) AS meal_created_count,
                COALESCE(follower_count.follower_count, 0) AS follower_count,
                COALESCE(following_count.following_count, 0) AS following_count,
                COALESCE(workout_count.workout_count, 0) AS workout_count,
                COALESCE(workout_day_count.workout_day_count, 0) AS workout_day_log_count,
                COALESCE(movement_count.movement_created_count, 0) AS movement_created_count,
                COALESCE(muscle_group_count.muscle_group_created_count, 0) AS muscle_group_created_count,
                COALESCE(meal_of_day_count.meal_of_day_created_count, 0) AS meal_of_day_created_count,
                COALESCE(meal_food_count.meal_food_created_count, 0) AS meal_food_created_count,
                COALESCE(exercise_count.exercise_count, 0) AS exercise_count,
                COALESCE(set_count.set_count, 0) AS set_count,
                COALESCE(rep_count.rep_count, 0) AS rep_count
            FROM
                users_user t1
                LEFT JOIN user_profile t2 ON t2.user_id = t1.id
                LEFT JOIN (
                    SELECT created_by_id, COUNT(*) AS food_created_count
                    FROM food
                    GROUP BY created_by_id
                ) food_count ON food_count.created_by_id = t1.id
                LEFT JOIN (
                    SELECT created_by_id, COUNT(*) AS brand_created_count
                    FROM food_brand
                    GROUP BY created_by_id
                ) brand_count ON brand_count.created_by_id = t1.id
                LEFT JOIN (
                    SELECT user_id, COUNT(*) AS diet_count
                    FROM food_log
                    GROUP BY user_id
                ) diet_count ON diet_count.user_id = t1.id
                LEFT JOIN (
                    SELECT user_id, COUNT(DISTINCT date) AS diet_day_count
                    FROM food_log
                    GROUP BY user_id
                ) diet_day_count ON diet_day_count.user_id = t1.id
                LEFT JOIN (
                    SELECT user_id, COUNT(*) AS diet_target_count
                    FROM diet_target
                    GROUP BY user_id
                ) diet_target_count ON diet_target_count.user_id = t1.id
                LEFT JOIN (
                    SELECT user_id, COUNT(*) AS progress_count
                    FROM progress
                    GROUP BY user_id
                ) progress_count ON progress_count.user_id = t1.id
                LEFT JOIN (
                    SELECT user_id, COUNT(*) AS meal_created_count
                    FROM meal
                    GROUP BY user_id
                ) meal_count ON meal_count.user_id = t1.id
                LEFT JOIN (
                    SELECT user_id, COUNT(*) AS follower_count
                    FROM user_follower
                    GROUP BY user_id
                ) follower_count ON follower_count.user_id = t1.id
                LEFT JOIN (
                    SELECT follower_id, COUNT(*) AS following_count
                    FROM user_follower
                    GROUP BY follower_id
                ) following_count ON following_count.follower_id = t1.id
                LEFT JOIN (
                    SELECT created_by_id, COUNT(*) AS movement_created_count
                    FROM movement
                    GROUP BY created_by_id
                ) movement_count ON movement_count.created_by_id = t1.id
                LEFT JOIN (
                    SELECT created_by_id, COUNT(*) AS muscle_group_created_count
                    FROM muscle_group
                    GROUP BY created_by_id
                ) muscle_group_count ON muscle_group_count.created_by_id = t1.id
                LEFT JOIN (
                    SELECT created_by_id, COUNT(*) AS meal_of_day_created_count
                    FROM meal_of_day
                    GROUP BY created_by_id
                ) meal_of_day_count ON meal_of_day_count.created_by_id = t1.id
                LEFT JOIN (
                    SELECT f2.user_id, COUNT(f1.id) AS meal_food_created_count
                    FROM meal_food f1
                    JOIN meal f2 ON f2.id = f1.meal_id
                    GROUP BY f2.user_id
                ) meal_food_count ON meal_food_count.user_id = t1.id
                LEFT JOIN (
                    SELECT user_id, COUNT(*) AS workout_count
                    FROM workout
                    GROUP BY user_id
                ) workout_count ON workout_count.user_id = t1.id
                LEFT JOIN (
                    SELECT user_id, COUNT(DISTINCT date) AS workout_day_count
                    FROM workout
                    GROUP BY user_id
                ) workout_day_count ON workout_day_count.user_id = t1.id
                LEFT JOIN (
                    SELECT e2.user_id, COUNT(e1.id) AS exercise_count
                    FROM exercise e1
                    JOIN workout e2 ON e2.id = e1.workout_id
                    GROUP BY e2.user_id
                ) exercise_count ON exercise_count.user_id = t1.id
                LEFT JOIN (
                    SELECT s3.user_id, COUNT(s1.id) AS set_count
                    FROM tracked_set s1
                    JOIN exercise s2 ON s2.id = s1.exercise_id
                    JOIN workout s3 ON s3.id = s2.workout_id
                    GROUP BY s3.user_id
                ) set_count ON set_count.user_id = t1.id
                LEFT JOIN (
                    SELECT r3.user_id, SUM(r1.reps) AS rep_count
                    FROM tracked_set r1
                    JOIN exercise r2 ON r2.id = r1.exercise_id
                    JOIN workout r3 ON r3.id = r2.workout_id
                    GROUP BY r3.user_id
                ) rep_count ON rep_count.user_id = t1.id
            WHERE
                TRUE
            ",
        );
        qb.filter("t1.username", "ilike", search);

        qb.push(" ORDER BY ");
        qb.push(order_by_column);

        qb.paginate(size, page);

        let query = qb.build_query_as().fetch_all(pool).await?;

        Ok(query)
    }
}
