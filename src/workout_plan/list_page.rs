use std::collections::HashSet;

use leptos::server_fn::codec::GetUrl;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::component::bulk_delete::{BulkDelete, BulkDeleteForm};
use crate::component::checkbox::CheckboxListItem;
use crate::component::input::FilterInput;
use crate::component::paginator::Paginator;
use crate::component::template::{
    AutoListHeader, ErrorComponent, ListLoadingComponent, ListNotFoundComponent,
    ListPageHeaderWithCreate,
};
use crate::util::datetime::format_datetime;
use crate::util::misc::ListResponse;
use crate::util::param::{extract_page, extract_param, extract_size};
use crate::workout_plan::create_page::WorkoutPlanCreate;
use crate::workout_plan::model::WorkoutPlanQuery;

#[cfg(feature = "ssr")]
use crate::auth::service::get_request_user;

#[server(endpoint = "workout-plan-list", input = GetUrl)]
pub async fn get_workout_plan_list(
    search: String,
    order: String,
    size: i64,
    page: i64,
) -> Result<ListResponse<WorkoutPlanQuery>, ServerFnError> {
    get_request_user()?;
    let pool = expect_context::<sqlx::PgPool>();
    let count = WorkoutPlanQuery::count(&pool, &search).await?;
    let results = WorkoutPlanQuery::filter(&pool, &search, &order, size, page).await?;
    Ok(ListResponse { count, results })
}

#[component]
pub fn WorkoutPlanListPage() -> impl IntoView {
    let action_bulk_delete = create_server_action::<BulkDelete>();
    let action_create = create_server_action::<WorkoutPlanCreate>();
    provide_context(action_create);

    let query = use_query_map();
    let search = move || extract_param(&query, "search");
    let order = move || extract_param(&query, "order");
    let size = move || extract_size(&query);
    let page = move || extract_page(&query);

    let resource = Resource::new(
        move || {
            (
                search(),
                order(),
                size(),
                page(),
                action_bulk_delete.version().get(),
                action_create.version().get(),
            )
        },
        |(search, order, size, page, _, _)| get_workout_plan_list(search, order, size, page),
    );

    let all_items = RwSignal::new(HashSet::<String>::new());
    let checked_items = RwSignal::new(HashSet::<String>::new());

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
                    .map(|inner| {
                        view! { <WorkoutPlanListItem data=inner checked_items/> }
                    })
                    .collect_view()
            }
        })
    };
    let count = move || {
        resource.with(|res| {
            res.as_ref()
                .and_then(|data| data.as_ref().ok().map(|res| res.count))
        })
    };

    view! {
        <Title text="Workout Plans"/>
        <main class="p-4 m-4 bg-white border">

            <ListPageHeaderWithCreate
                title="Workout Plans"
                create_href="/training-plans/workout-plans/create"
            >
                <Transition>{count}</Transition>
            </ListPageHeaderWithCreate>

            <section>
                <Form method="get" action="" class="flex flex-wrap gap-2">
                    <FilterInput name="search" value=Signal::derive(search)/>
                    <FilterInput name="order" value=Signal::derive(order)/>
                </Form>
            </section>

            <section class="grid mb-4 grid-cols-checkbox-10">
                <AutoListHeader all_items checked_items>
                    "Workout Plan"
                    ""
                    "Exercises"
                    "Sets"
                    "Reps"
                    "Weekday"
                    "Sequence"
                    "Training Plan"
                    "Created"
                    "Updated"
                </AutoListHeader>
                <Transition fallback=ListLoadingComponent>
                    <ErrorBoundary fallback=|errors| {
                        view! { <ErrorComponent errors/> }
                    }>{response}</ErrorBoundary>
                </Transition>
            </section>

            <section class="flex flex-wrap">
                <div>
                    <BulkDeleteForm table="workout_plan" action=action_bulk_delete checked_items/>
                </div>
                <div class="flex-1">
                    <Form method="GET" action="" class="contents">
                        <input type="hidden" name="search" value=search/>
                        <input type="hidden" name="order" value=order/>
                        <input type="hidden" name="page" value=page/>
                        <Transition>
                            <Paginator count/>
                        </Transition>
                    </Form>
                </div>
            </section>

        </main>
    }
}

#[component]
pub fn WorkoutPlanListItem<'a>(
    data: &'a WorkoutPlanQuery,
    checked_items: RwSignal<HashSet<String>>,
) -> impl IntoView {
    let created_at = format_datetime(&Some(data.created_at));
    let updated_at = format_datetime(&data.updated_at);

    let training_plan_name = data
        .training_plan_name
        .as_ref()
        .map_or_else(|| "-".to_string(), |t| t.clone());

    let name = data.name.clone();
    let slug = data.slug.clone();

    view! {
        <div class="contents group">
            <div class="flex justify-center items-center border-b group-hover:bg-gray-200 group-odd:bg-gray-50">
                <CheckboxListItem id=data.id.to_string() checked_items/>
            </div>
            <div class="flex col-span-2 items-center p-2 border-b group-hover:bg-gray-200 group-odd:bg-gray-50">
                <A href=slug>{name}</A>
            </div>
            <div class="flex items-center p-2 border-b group-hover:bg-gray-200 group-odd:bg-gray-50">
                {data.exercise_count}
            </div>
            <div class="flex items-center p-2 border-b group-hover:bg-gray-200 group-odd:bg-gray-50">
                {data.set_count}
            </div>
            <div class="flex items-center p-2 border-b group-hover:bg-gray-200 group-odd:bg-gray-50">
                {data.rep_count}
            </div>
            <div class="flex items-center p-2 border-b group-hover:bg-gray-200 group-odd:bg-gray-50">
                {data.weekday.as_ref().map_or_else(|| "-".to_string(), |w| w.to_string())}
            </div>
            <div class="flex items-center p-2 border-b group-hover:bg-gray-200 group-odd:bg-gray-50">
                {data.sequence.map_or_else(|| "-".to_string(), |s| s.to_string())}
            </div>
            <div class="flex items-center p-2 border-b group-hover:bg-gray-200 group-odd:bg-gray-50 truncate">
                {training_plan_name}
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
