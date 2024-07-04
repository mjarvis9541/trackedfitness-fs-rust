use std::collections::HashSet;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::component::badge::{Badge, BadgeVariant};
use crate::component::bulk_delete::BulkDeleteForm;
use crate::component::checkbox::CheckboxListItem;
use crate::component::template::{
    AutoListHeader, ErrorComponent, ListNotFoundComponent, LoadingComponent, UpdateDeleteButtonRow,
};
use crate::exercise_plan::create_page::{ExercisePlanCreate, ExercisePlanCreateForm};
use crate::exercise_plan::model::ExercisePlanQuery;
use crate::exercise_plan::update_page::ExercisePlanUpdate;
use crate::movement::select::get_movement_select;
use crate::training_plan::router::WorkoutPlanDetailParam;
use crate::util::datetime::format_datetime;
use crate::workout_plan::model::WorkoutPlanQuery;

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, error::Error, setup::get_pool};

#[server(endpoint = "workout-plan-detail")]
pub async fn get_workout_plan_detail(slug: String) -> Result<WorkoutPlanQuery, ServerFnError> {
    let _user = get_request_user()?;
    let pool = get_pool()?;

    let result = WorkoutPlanQuery::get_with_exercise_plans(&pool, &slug)
        .await?
        .ok_or(Error::NotFound)?;
    Ok(result)
}

#[component]
pub fn WorkoutPlanDetailPage() -> impl IntoView {
    let action_bulk_delete = Action::server();
    let action_exercise_plan_update = Action::<ExercisePlanUpdate, _>::server();
    let action_exercise_plan_create = Action::<ExercisePlanCreate, _>::server();
    provide_context(action_exercise_plan_create);
    provide_context(action_exercise_plan_update);
    let params = use_params::<WorkoutPlanDetailParam>();
    let slug = move || {
        params.with(|p| {
            p.as_ref()
                .map(|p| p.workout_slug.clone())
                .unwrap_or_default()
        })
    };
    let movement_resource = Resource::once(get_movement_select);
    provide_context(movement_resource);

    let resource = Resource::new(
        move || {
            (
                slug(),
                action_bulk_delete.version().get(),
                action_exercise_plan_create.version().get(),
                action_exercise_plan_update.version().get(),
            )
        },
        |(slug, ..)| get_workout_plan_detail(slug),
    );

    let workout_plan_id = RwSignal::new(String::new());
    let all_items = RwSignal::new(HashSet::<String>::new());
    let checked_items = RwSignal::new(HashSet::<String>::new());

    let checked_len = move || checked_items.with(|v| v.len());

    let response = move || {
        resource.and_then(|data| {
            let exercises = &data.exercise_plans;
            let ids: HashSet<String> = exercises.iter().map(|item| item.id.to_string()).collect();
            all_items.update(|set| set.extend(ids));
            workout_plan_id.update(|v| *v = data.id.to_string());
            if exercises.is_empty() {
                view! { <ListNotFoundComponent/> }
            } else {
                exercises
                    .iter()
                    .map(|inner| {
                        view! { <WorkoutDetailExercisePlanListItem data=inner checked_items/> }
                    })
                    .collect_view()
            }
        })
    };
    let workout_response = move || {
        resource.and_then(|data| {
            let data = data.clone();
            view! { <WorkoutPlanDetailItem data/> }
        })
    };
    view! {
        <Title text="Workout Plan"/>
        <main class="p-4 m-4 bg-white border">
            <Transition fallback=LoadingComponent>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors/> }
                }>{workout_response}</ErrorBoundary>
            </Transition>

            <section class="grid mb-4 grid-cols-checkbox-8">
                <AutoListHeader all_items checked_items>
                    "Exercise"
                    ""
                    "Sets"
                    "Reps"
                    "Workout Plan"
                    "Created"
                    "Updated"
                    "Sequence"
                </AutoListHeader>
                <Transition fallback=LoadingComponent>
                    <ErrorBoundary fallback=|errors| {
                        view! { <ErrorComponent errors/> }
                    }>{response}</ErrorBoundary>
                </Transition>
            </section>

            <section class="flex gap-4 items-center">
                <div>
                    <BulkDeleteForm table="exercise_plan" action=action_bulk_delete checked_items/>
                </div>
                <div>"Selected: " {checked_len}</div>
            </section>

        </main>
        <section class="p-4 m-4 max-w-md bg-white border">
            <ExercisePlanCreateForm workout_plan_id/>
        </section>
    }
}

#[component]
pub fn WorkoutPlanDetailItem(data: WorkoutPlanQuery) -> impl IntoView {
    view! {
        <header class="flex justify-between items-start">
            <section class="flex-1">
                <p>{data.weekday.as_ref().map(|w| w.to_string())}</p>
                <h2 class="text-base font-bold">{data.sequence} ". " {&data.name}</h2>
            </section>

            <section class="flex flex-wrap gap-2 items-start">
                <Badge variant=BadgeVariant::Tertiary label="Sets" value=data.set_count/>
                <Badge variant=BadgeVariant::Tertiary label="Reps" value=data.rep_count/>
                <UpdateDeleteButtonRow/>
            </section>
        </header>
    }
}

#[component]
pub fn WorkoutDetailExercisePlanListItem<'a>(
    data: &'a ExercisePlanQuery,
    checked_items: RwSignal<HashSet<String>>,
) -> impl IntoView {
    let created_at = format_datetime(&Some(data.created_at));
    let updated_at = format_datetime(&data.updated_at);

    let exercise_id = data.id.to_string();
    let exercise_name = data.movement_name.clone();

    view! {
        <div class="contents group">
            <div class="flex justify-center items-center border-b group-hover:bg-gray-200 group-odd:bg-gray-50">
                <CheckboxListItem id=data.id.to_string() checked_items/>
            </div>
            <div class="flex col-span-2 items-center p-2 border-b group-hover:bg-gray-200 group-odd:bg-gray-50 truncate">
                <A href=exercise_id>{exercise_name}</A>
            </div>
            <div class="flex items-center p-2 border-b group-hover:bg-gray-200 group-odd:bg-gray-50">
                {data.sets}
            </div>
            <div class="flex items-center p-2 border-b group-hover:bg-gray-200 group-odd:bg-gray-50">
                {data.reps}
            </div>
            <div class="flex items-center p-2 border-b group-hover:bg-gray-200 group-odd:bg-gray-50">
                {data.sequence}
            </div>
            <div class="flex items-center p-2 border-b group-hover:bg-gray-200 group-odd:bg-gray-50 truncate">
                {&data.workout_plan_name}
            </div>
            <div class="flex items-center p-2 border-b group-hover:bg-gray-200 group-odd:bg-gray-50 truncate">
                {created_at}
            </div>
            <div class="flex items-center p-2 border-b group-hover:bg-gray-200 group-odd:bg-gray-50 truncate">
                {updated_at}
            </div>
        </div>
    }
}
