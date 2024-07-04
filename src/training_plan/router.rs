use leptos::*;
use leptos_router::*;

use uuid::Uuid;

use crate::component::link::{Link, LinkVariant};
use crate::exercise_plan::delete_page::ExercisePlanDeletePage;
use crate::exercise_plan::detail_page::ExercisePlanDetailPage;
use crate::exercise_plan::list_page::ExercisePlanListPage;
use crate::exercise_plan::update_page::ExercisePlanUpdatePage;
use crate::training_plan::create_page::TrainingPlanCreatePage;
use crate::training_plan::delete_page::TrainingPlanDeletePage;
use crate::training_plan::detail_page::TrainingPlanDetailPage;
use crate::training_plan::list_page::TrainingPlanListPage;
use crate::training_plan::update_page::TrainingPlanUpdatePage;
use crate::workout_plan::create_page::WorkoutPlanCreatePage;
use crate::workout_plan::delete_page::WorkoutPlanDeletePage;
use crate::workout_plan::detail_page::WorkoutPlanDetailPage;
use crate::workout_plan::list_page::WorkoutPlanListPage;
use crate::workout_plan::update_page::WorkoutPlanUpdatePage;

#[derive(Debug, PartialEq, Params)]
pub struct TrainingPlanDetailParam {
    pub training_slug: String,
}

#[derive(Debug, PartialEq, Params)]
pub struct WorkoutPlanDetailParam {
    pub training_slug: Option<String>,
    pub workout_slug: String,
}

#[derive(Debug, PartialEq, Params)]
pub struct ExercisePlanDetailParam {
    pub training_slug: Option<String>,
    pub workout_slug: String,
    pub exercise_id: Uuid,
}

#[component(transparent)]
pub fn TrainingPlanRouter() -> impl IntoView {
    view! {
        <Route path="/training-plans" view=TrainingPlanLayout>
            <Route path="/create" view=TrainingPlanCreatePage/>
            <Route path="/:training_slug" view=TrainingPlanDetailPage/>
            <Route path="/:training_slug/update" view=TrainingPlanUpdatePage/>
            <Route path="/:training_slug/delete" view=TrainingPlanDeletePage/>
            <Route path="/:training_slug/:workout_slug" view=WorkoutPlanDetailPage/>
            <Route path="/:training_slug/:workout_slug/update" view=WorkoutPlanUpdatePage/>
            <Route path="/:training_slug/:workout_slug/delete" view=WorkoutPlanDeletePage/>
            <Route path="/:training_slug/:workout_slug/:exercise_id" view=ExercisePlanDetailPage/>
            <Route
                path="/:training_slug/:workout_slug/:exercise_id/update"
                view=ExercisePlanUpdatePage
            />
            <Route
                path="/:training_slug/:workout_slug/:exercise_id/delete"
                view=ExercisePlanDeletePage
            />

            // workout plans
            <Route path="/workout-plans" view=WorkoutPlanListPage/>
            <Route path="/workout-plans/create" view=WorkoutPlanCreatePage/>
            <Route path="/workout-plans/:workout_slug" view=WorkoutPlanDetailPage/>
            <Route path="/workout-plans/:workout_slug/update" view=WorkoutPlanUpdatePage/>
            <Route path="/workout-plans/:workout_slug/delete" view=WorkoutPlanDeletePage/>
            <Route path="/workout-plans/:workout_slug/:exercise_id" view=ExercisePlanDetailPage/>
            <Route
                path="/workout-plans/:workout_slug/:exercise_id/update"
                view=ExercisePlanUpdatePage
            />
            <Route
                path="/workout-plans/:workout_slug/:exercise_id/delete"
                view=ExercisePlanDeletePage
            />

            // exercise plans
            <Route path="/exercise-plans" view=ExercisePlanListPage/>
            <Route path="/" view=TrainingPlanListPage/>
        </Route>
    }
}

#[component]
pub fn TrainingPlanLayout() -> impl IntoView {
    view! {
        <nav class="flex">
            <Link exact=true variant=LinkVariant::Wide text="Training Plans" href=""/>
            <Link exact=true variant=LinkVariant::Wide text="Workout Plans" href="workout-plans"/>
            <Link exact=true variant=LinkVariant::Wide text="Exercise Plans" href="exercise-plans"/>
        </nav>
        <Outlet/>
    }
}
