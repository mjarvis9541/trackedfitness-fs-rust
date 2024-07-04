use leptos::*;
use leptos_router::*;

use chrono::NaiveDate;
use uuid::Uuid;

use crate::exercise::delete_page::ExerciseDeletePage;
use crate::exercise::detail_page::ExerciseDetailPage;
use crate::exercise::update_page::ExerciseUpdatePage;
use crate::set::delete_page::SetDeletePage;
use crate::set::detail_page::SetDetailPage;
use crate::set::update_page::SetUpdatePage;
use crate::workout::day_page::WorkoutDayPage;
use crate::workout::delete_page::WorkoutDeletePage;
use crate::workout::detail_page::WorkoutDetailPage;
use crate::workout::exercise_set_create_page::ExerciseSetCreatePage;
use crate::workout::from_plan_create_page::WorkoutCreateFromPlanListPage;
use crate::workout::layout::WorkoutLayout;
use crate::workout::multi_create::WorkoutExerciseSetCreatePage;
use crate::workout::update_page::WorkoutUpdatePage;

#[derive(Debug, PartialEq, Params)]
pub struct UsernameParam {
    pub username: String,
}

#[derive(Debug, PartialEq, Params)]
pub struct UsernameDateParam {
    pub username: String,
    pub date: Option<NaiveDate>,
}

#[derive(Debug, PartialEq, Params)]
pub struct WorkoutDetailParam {
    pub username: String,
    pub date: NaiveDate,
    pub workout_id: Uuid,
}

#[derive(Debug, PartialEq, Params)]
pub struct ExerciseDetailParam {
    pub username: String,
    pub date: NaiveDate,
    pub workout_id: Uuid,
    pub exercise_id: Uuid,
}

#[derive(Debug, PartialEq, Params)]
pub struct SetDetailParam {
    pub username: String,
    pub date: NaiveDate,
    pub workout_id: Uuid,
    pub exercise_id: Uuid,
    pub set_id: Uuid,
}

#[component(transparent)]
pub fn WorkoutRouter() -> impl IntoView {
    view! {
        <Route path="/workouts" view=WorkoutLayout>
            <Route path="/:date?" view=WorkoutDayPage/>
            <Route path="/:date/create-with-exercise" view=WorkoutExerciseSetCreatePage/>
            <Route path="/:date/:workout_id" view=WorkoutDetailPage/>
            <Route path="/:date/:workout_id/update" view=WorkoutUpdatePage/>
            <Route path="/:date/:workout_id/delete" view=WorkoutDeletePage/>
            <Route path="/:date/:workout_id/add-exercise" view=ExerciseSetCreatePage/>
            <Route path="/:date/:workout_id/add-workout-plan" view=WorkoutCreateFromPlanListPage/>
            <Route path="/:date/:workout_id/:exercise_id" view=ExerciseDetailPage/>
            <Route path="/:date/:workout_id/:exercise_id/update" view=ExerciseUpdatePage/>
            <Route path="/:date/:workout_id/:exercise_id/delete" view=ExerciseDeletePage/>
            <Route path="/:date/:workout_id/:exercise_id/:set_id" view=SetDetailPage/>
            <Route path="/:date/:workout_id/:exercise_id/:set_id/update" view=SetUpdatePage/>
            <Route path="/:date/:workout_id/:exercise_id/:set_id/delete" view=SetDeletePage/>
        </Route>
    }
}
