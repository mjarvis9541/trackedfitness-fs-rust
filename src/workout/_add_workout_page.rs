use std::collections::HashSet;

use leptos::server_fn::codec::GetUrl;
use leptos::*;
use leptos_meta::Title;
use leptos_router::*;
use uuid::Uuid;

use crate::component::bulk_delete::BulkDeleteForm;
use crate::component::button::Button;
use crate::component::checkbox::CheckboxListItem;
use crate::component::input::FilterInput;
use crate::component::template::{
    AutoListHeader, ErrorComponent, ListLoadingComponent, ListNotFoundComponent,
};
use crate::error_extract::extract_error_message;
use crate::exercise_plan::model::ExercisePlanQuery;
use crate::util::datetime::format_datetime;
use crate::util::misc::ListResponse;
use crate::util::param::extract_param;
use crate::workout::router::WorkoutDetailParam;
use crate::workout_plan::model::WorkoutPlanQuery;

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user,
    exercise::model::ExerciseBase,
    exercise_plan::model::ExercisePlan,
    set::model::{MergedSetInputData, SetBase, SetToCreate},
    setup::get_pool,
    util::server::parse_uuids_from_strings,
};

#[server(endpoint = "workout-plan-list-with-exercise-plans", input = GetUrl)]
pub async fn get_workout_plan_list_with_exercises(
    search: String,
) -> Result<ListResponse<WorkoutPlanQuery>, ServerFnError> {
    let _user = get_request_user()?;
    let pool = get_pool()?;
    let count = WorkoutPlanQuery::count(&pool, &search).await?;
    let results = WorkoutPlanQuery::get_all_with_exercise_plans(&pool, &search).await?;
    Ok(ListResponse { count, results })
}

#[server(endpoint = "exercise-from-plan-create")]
pub async fn exercise_log_bulk_create(
    workout_id: Uuid,
    workout_plan_ids: Option<HashSet<String>>,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    let items = workout_plan_ids.ok_or_else(|| ServerFnError::new("No workout plans provided"))?;
    let workout_plan_ids = parse_uuids_from_strings(&items)?;
    if workout_plan_ids.is_empty() {
        return Err(ServerFnError::new("Invalid workout plan IDs"));
    }
    let exercise_plans =
        ExercisePlan::get_all_by_workout_plan_ids(&pool, &workout_plan_ids).await?;
    if exercise_plans.is_empty() {
        return Err(ServerFnError::new("No exercises found"));
    }
    let sets_to_create = SetToCreate::from_exercise_plan(&exercise_plans);
    let exercises =
        ExerciseBase::bulk_create_from_exercise_plan(&pool, workout_id, &exercise_plans, user.id)
            .await?;
    let merged_data = MergedSetInputData::merge_from_exercise_sets(&exercises, &sets_to_create);
    SetBase::bulk_create_from_set_input_data_vec(&pool, &merged_data, user.id).await?;
    Ok(())
}

#[component]
pub fn WorkoutCreateFromPlanListPage() -> impl IntoView {
    let action_bulk_create = Action::<ExerciseLogBulkCreate, _>::server();
    let action_bulk_delete = Action::server();
    let error = move || extract_error_message(&action_bulk_create);

    let params = use_params::<WorkoutDetailParam>();
    let workout_id = move || {
        params.with(|q| {
            q.as_ref()
                .map(|q| q.workout_id)
                .unwrap_or_default()
                .to_string()
        })
    };

    let query = use_query_map();
    let search = move || extract_param(&query, "search");

    let resource = Resource::new(
        move || (search(), action_bulk_delete.version().get()),
        |(search, _)| get_workout_plan_list_with_exercises(search),
    );

    let all_items = RwSignal::new(HashSet::<String>::new());
    let checked_items = RwSignal::new(HashSet::<String>::new());

    let exercise_checked_items: RwSignal<HashSet<String>> = RwSignal::new(HashSet::<String>::new());
    provide_context(exercise_checked_items);

    let response = move || {
        resource.and_then(|data| {
            let count = &data.count;
            let results = &data.results;
            if *count == 0 {
                view! { <ListNotFoundComponent/> }
            } else {
                let ids: HashSet<String> = results.iter().map(|item| item.id.to_string()).collect();
                all_items.update(|set| set.extend(ids));
                results
                    .iter()
                    .enumerate()
                    .map(|(key, inner)| {
                        view! { <WorkoutPlanListItem key=key data=inner checked_items/> }
                    })
                    .collect_view()
            }
        })
    };

    // let order_options: &'static [(&'static str, &'static str)] = &[
    //     ("t1.name", "Name (A-z)"),
    //     ("-t1.name", "Name (Z-a)"),
    //     ("t1.created_at", "Created (Asc)"),
    //     ("-t1.created_at", "Created (Desc)"),
    //     ("t1.updated_at", "Updated (Asc)"),
    //     ("-t1.updated_at", "Updated (Desc)"),
    // ];

    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        if let Ok(mut data) = ExerciseLogBulkCreate::from_event(&ev) {
            data.workout_plan_ids = Some(checked_items.get());
            checked_items.update(|v| v.clear());
            action_bulk_create.dispatch(data)
        }
    };

    view! {
        <Title text="Add Workout Plan"/>
        <main class="p-4">

            <section class="border">
                <ActionForm action=action_bulk_create on:submit=handle_submit>
                    <input type="hidden" name="workout_id" value=workout_id/>
                    <input type="hidden" name="workout_plan_ids" value=""/>
                    <Button
                        label="Add to Log"
                        loading=action_bulk_create.pending()
                        disabled=Signal::derive(move || checked_items.with(HashSet::is_empty))
                    />
                </ActionForm>
                {error}
            </section>

            <section class="flex flex-wrap gap-2 mb-4 lg:mb-2">

                <Form method="GET" action="" class="contents">
                    <FilterInput name="search" value=Signal::derive(search)/>

                </Form>
            </section>

            <section class="grid grid-cols-checkbox-8">
                <AutoListHeader all_items checked_items>
                    "Workout Plan"
                    ""
                    "Exercises"
                    "Sets"
                    "Reps"
                    "Weight"
                    "Created"
                    "Updated"
                </AutoListHeader>
                <Transition fallback=ListLoadingComponent>
                    <ErrorBoundary fallback=|errors| {
                        view! { <ErrorComponent errors/> }
                    }>{response}</ErrorBoundary>
                </Transition>
            </section>

            <section class="flex flex-wrap pt-4">
                <div class="hidden md:block">
                    <BulkDeleteForm table="workout_plan" action=action_bulk_delete checked_items/>
                </div>

            </section>

            <section class="my-4">{move || format!("{:?}", checked_items.get())}</section>
            <section class="my-4">{move || format!("{:?}", exercise_checked_items.get())}</section>

        </main>
    }
}

#[component]
pub fn WorkoutPlanListItem<'a>(
    key: usize,
    data: &'a WorkoutPlanQuery,
    checked_items: RwSignal<HashSet<String>>,
) -> impl IntoView {
    let created_at = format_datetime(&Some(data.created_at));
    let updated_at = format_datetime(&data.updated_at);

    let name = data.name.clone();

    let hide_exercises = RwSignal::new(true);
    let toggle_exercises = move |_| hide_exercises.update(|v| *v = !*v);

    let exercises = &data.exercise_plans;
    let exercises_view = exercises
        .iter()
        .map(|exercise_plan| {
            view! { <ExercisePlanListItem data=exercise_plan/> }
        })
        .collect_view();

    let is_even = key % 2 == 0;

    let count = data.exercise_count.clone();
    let disabled = move || count == 0;
    let no_exercises = Signal::derive(disabled);

    view! {
        <div class="contents group">
            <div
                class="flex justify-center items-center group-hover:bg-gray-200"
                class=("bg-gray-50", is_even)
            >
                <CheckboxListItem id=data.id.to_string() checked_items disabled=no_exercises/>
            </div>
            <div
                class="flex col-span-2 items-center group-hover:bg-gray-200"
                class=("bg-gray-50", is_even)
            >
                <button
                    on:click=toggle_exercises
                    class="flex flex-1 py-1 px-2 bg-gray-200 disabled:opacity-50"
                    class=("border-blue-500", no_exercises.with(|v| *v == false))
                    disabled=no_exercises
                >
                    {name}
                </button>
            </div>
            <div
                class="flex items-center p-2 group-hover:bg-gray-200"
                class=("bg-gray-50", is_even)
            >
                {data.exercise_count}
            </div>
            <div
                class="flex items-center p-2 group-hover:bg-gray-200"
                class=("bg-gray-50", is_even)
            >
                {data.set_count}
            </div>
            <div
                class="flex items-center p-2 group-hover:bg-gray-200"
                class=("bg-gray-50", is_even)
            >
                {data.rep_count}
            </div>
            <div
                class="flex items-center p-2 group-hover:bg-gray-200"
                class=("bg-gray-50", is_even)
            ></div>
            <div
                class="flex items-center p-2 group-hover:bg-gray-200 truncate"
                class=("bg-gray-50", is_even)
            >
                {created_at}
            </div>
            <div
                class="flex items-center p-2 group-hover:bg-gray-200 truncate"
                class=("bg-gray-50", is_even)
            >
                {updated_at}
            </div>
        </div>
        <div class="contents" class=("hidden", hide_exercises)>
            {exercises_view}
        </div>
    }
}

#[component]
pub fn ExercisePlanListItem<'a>(data: &'a ExercisePlanQuery) -> impl IntoView {
    let exercise_checked_items = expect_context::<RwSignal<HashSet<String>>>();
    let created_at = format_datetime(&Some(data.created_at));
    let updated_at = format_datetime(&data.updated_at);

    let name = data.movement_name.clone();
    let slug = data.id;
    let update_href = format!("{}/update", slug);
    let weight = format!("{:.2}kg", data.weight);

    view! {
        <div class="contents group/exercise">
            <div class="flex justify-center items-center px-2 group-hover:bg-gray-200 group-odd/exercise:bg-gray-50">
                <CheckboxListItem id=data.id.to_string() checked_items=exercise_checked_items/>
            </div>
            <div class="flex col-span-2 items-center p-2 group-hover:bg-gray-200 group-odd/exercise:bg-gray-50 truncate">
                <A href=update_href>{name}</A>
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd/exercise:bg-gray-50">
                "-"
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd/exercise:bg-gray-50">
                {data.sets}
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd/exercise:bg-gray-50">
                {data.reps}
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd/exercise:bg-gray-50 truncate">
                {weight}
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd/exercise:bg-gray-50 truncate">
                {created_at}
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd/exercise:bg-gray-50 truncate">
                {updated_at}
            </div>
        </div>
    }
}
