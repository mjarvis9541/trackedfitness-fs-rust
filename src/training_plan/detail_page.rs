use leptos::server_fn::codec::GetUrl;
use leptos::*;
use leptos_meta::Title;
use leptos_router::*;

use crate::component::icon::{IconEditA, IconTrash};
use crate::component::link::{Link, LinkVariant};
use crate::component::template::{ErrorComponent, LoadingComponent};
use crate::exercise_plan::create_page::{AddExercisePlanForm, ExercisePlanCreate};
use crate::exercise_plan::delete_page::{ExercisePlanDelete, ExercisePlanDeleteForm};
use crate::exercise_plan::model::ExercisePlanQuery;
use crate::movement::select::{get_movement_select, MoveSelectResource};
use crate::training_plan::create_page::TrainingPlanCreate;
use crate::training_plan::delete_page::TrainingPlanDelete;
use crate::training_plan::model::TrainingPlanQuery;
use crate::util::param::extract_param;
use crate::workout_plan::create_page::{WorkoutPlanAddForm, WorkoutPlanCreate};
use crate::workout_plan::delete_page::{WorkoutPlanDelete, WorkoutPlanDeleteForm};
use crate::workout_plan::model::WorkoutPlanQuery;

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, error::Error};

#[server(endpoint = "training-plan-detail", input = GetUrl)]
pub async fn get_training_plan_detail(slug: String) -> Result<TrainingPlanQuery, ServerFnError> {
    let _token = get_request_user()?;
    let pool = expect_context::<sqlx::PgPool>();
    let query = TrainingPlanQuery::get_with_workout_plans(&pool, &slug)
        .await?
        .ok_or(Error::NotFound)?;
    Ok(query)
}

#[component]
pub fn TrainingPlanDetailPage() -> impl IntoView {
    let action_training_plan_create = Action::<TrainingPlanCreate, _>::server();
    let action_training_plan_delete = Action::<TrainingPlanDelete, _>::server();
    let action_workout_plan_create = Action::<WorkoutPlanCreate, _>::server();
    let action_workout_plan_delete = Action::<WorkoutPlanDelete, _>::server();
    let action_exercise_plan_create = Action::<ExercisePlanCreate, _>::server();
    let action_exercise_plan_delete = Action::<ExercisePlanDelete, _>::server();

    provide_context(action_training_plan_create);
    provide_context(action_training_plan_delete);
    provide_context(action_workout_plan_create);
    provide_context(action_workout_plan_delete);
    provide_context(action_exercise_plan_create);
    provide_context(action_exercise_plan_delete);

    let params = use_params_map();
    let slug = move || extract_param(&params, "training_slug");

    let movement_resource: MoveSelectResource = Resource::once(get_movement_select);
    provide_context(movement_resource);

    let resource = Resource::new(
        move || {
            (
                slug(),
                action_workout_plan_create.version().get(),
                action_workout_plan_delete.version().get(),
                action_exercise_plan_create.version().get(),
                action_exercise_plan_delete.version().get(),
            )
        },
        |(slug, ..)| get_training_plan_detail(slug),
    );
    let response = move || resource.and_then(|data| view! { <TrainingPlanComponent data/> });

    view! {
        <Title text="Training Plan"/>
        <main class="p-4 bg-white border">

            <Transition fallback=LoadingComponent>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors/> }
                }>{response}</ErrorBoundary>
            </Transition>
        </main>
    }
}

#[component]
pub fn TrainingPlanComponent<'a>(data: &'a TrainingPlanQuery) -> impl IntoView {
    let training_plan_id = data.id.to_string();
    let next_workout_seq = (data.workout_count + 1).to_string();

    let workout_plans = &data.workout_plans;
    let workout_plans_view = workout_plans
        .iter()
        .map(|workout| {
            view! { <WorkoutComponent data=workout/> }
        })
        .collect_view();

    view! {
        <header class="flex flex-wrap gap-2 justify-between items-start p-2 mb-2 hover:bg-amber-200 bg-emerald-600/50">
            <section>
                <h1 class="text-xl font-bold capitalize">{&data.name}</h1>
                <section class="flex gap-2">
                    <div class="font-bold">{data.workout_count} " workouts"</div>
                    <div class="font-bold">"|"</div>
                    <div class="font-bold">{data.exercise_count} " exercises"</div>
                    <div class="font-bold">"|"</div>
                    <div class="font-bold">{data.set_count} " sets"</div>
                    <div class="font-bold">"|"</div>
                    <div class="font-bold">{data.rep_count} " reps"</div>
                    <div class="">"-"</div>
                    <div>"Duration: " {data.duration_weeks} " weeks"</div>
                </section>
            </section>
            <section class="flex gap-2 justify-end">
                <Link href="update" variant=LinkVariant::Secondary>
                    <IconEditA/>
                </Link>
                <Link href="delete" variant=LinkVariant::Danger>
                    <IconTrash/>
                </Link>
            </section>
        </header>

        <section class="grid grid-flow-col auto-cols-[20%] gap-2 overflow-x-auto overscroll-x-contain mb-2">
            {workout_plans_view}
        </section>

        <section class="p-2 max-w-sm bg-gray-100 border">
            <h2 class="mb-2 text-base font-bold">"Add Workout"</h2>
            <WorkoutPlanAddForm training_plan_id next_workout_seq class="space-y-4"/>
        </section>
    }
}

#[component]
pub fn WorkoutComponent<'a>(data: &'a WorkoutPlanQuery) -> impl IntoView {
    let title = data.sequence.map_or_else(
        || data.name.to_string(),
        |s| format!("{}. {}", s, data.name),
    );

    let update_href = format!("/training-plans/workout-plans/{}/update", data.slug);

    let workout_slug = &data.slug;
    let workout_plan_id = data.id.to_string();
    let workout_plan_id_b = workout_plan_id.clone();

    let next_exercise_seq = (data.exercise_count + 1).to_string();

    let exercise_plans = &data.exercise_plans;
    let exercise_plans_view = exercise_plans
        .iter()
        .map(|exercise| {
            view! { <ExerciseComponent exercise workout_slug/> }
        })
        .collect_view();

    view! {
        <div>
            <header class="flex justify-between p-2 mb-2 bg-gray-300 border hover:bg-amber-200">
                <section>
                    <div>{data.weekday.as_ref().map(|w| w.to_string())}</div>
                    <h2 class="font-bold capitalize hover:underline">
                        <A href=update_href>{title}</A>
                    </h2>
                    <div class="flex gap-2">
                        <div>{data.exercise_count} " exercises"</div>
                        <div>"|"</div>
                        <div>{data.set_count} " sets"</div>
                        <div>"|"</div>
                        <div>{data.rep_count} " reps"</div>
                    </div>
                </section>

                <section class="flex gap-4">
                    <WorkoutPlanDeleteForm id=workout_plan_id/>
                </section>
            </header>

            <section>{exercise_plans_view}</section>

            <section class="p-2 bg-gray-200">
                <AddExercisePlanForm workout_plan_id=workout_plan_id_b next_exercise_seq/>
            </section>
        </div>
    }
}

#[component]
pub fn ExerciseComponent<'a>(
    exercise: &'a ExercisePlanQuery,
    workout_slug: &'a String,
) -> impl IntoView {
    let title = format!("{}. {}", exercise.sequence, exercise.movement_name);
    let text = format!("{} sets of {} reps.", exercise.sets, exercise.reps);
    let exercise_plan_id = exercise.id.to_string();
    let update_href = format!("{}/{}/update", workout_slug, exercise.id);
    view! {
        <div class="flex justify-between p-2 mb-2 bg-gray-100 border hover:bg-amber-200">
            <section>
                <h3 class="mb-1 font-bold hover:underline">
                    <A href=update_href>{title}</A>
                </h3>
                <div>{text}</div>
            </section>
            <section>
                <ExercisePlanDeleteForm id=exercise_plan_id/>
            </section>
        </div>
    }
}
