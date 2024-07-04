use std::collections::HashSet;

use leptos::server_fn::codec::GetUrl;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::component::bulk_delete::BulkDeleteForm;
use crate::component::checkbox::CheckboxListItem;
use crate::component::input::FilterInput;
use crate::component::paginator::Paginator;
use crate::component::template::{
    AutoListHeader, ErrorComponent, ListLoadingComponent, ListNotFoundComponent,
};
use crate::exercise_plan::model::ExercisePlanQuery;
use crate::util::datetime::format_datetime;
use crate::util::misc::ListResponse;
use crate::util::param::{extract_page, extract_param, extract_size};

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, setup::get_pool};

#[server(endpoint = "exercise-plan-list", input = GetUrl)]
pub async fn get_exercise_plan_list(
    search: String,
    order: String,
    size: i64,
    page: i64,
) -> Result<ListResponse<ExercisePlanQuery>, ServerFnError> {
    let _user = get_request_user()?;
    let pool = get_pool()?;
    let count = ExercisePlanQuery::count(&pool, &search).await?;
    let results = ExercisePlanQuery::filter(&pool, &search, &order, size, page).await?;
    Ok(ListResponse { count, results })
}

#[component]
pub fn ExercisePlanListPage() -> impl IntoView {
    let action_bulk_delete = Action::server();

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
            )
        },
        |(search, order, size, page, _)| get_exercise_plan_list(search, order, size, page),
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
                        view! { <ExercisePlanListItem data=inner checked_items/> }
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
        <Title text="Exercise Plans"/>
        <main class="grid grid-cols-4 gap-4 p-4 lg:grid-cols-12">

            <div class="col-span-4 p-4 bg-white border lg:col-span-12">

                <header class="mb-2">
                    <h1 class="text-xl font-bold">"Exercise Plans"</h1>
                    <p class="text-gray-400">"Results: " <Transition>{count}</Transition></p>
                </header>

                <section>
                    <Form method="get" action="" class="flex flex-wrap gap-2">
                        <FilterInput name="search" value=Signal::derive(search)/>
                        <FilterInput name="order" value=Signal::derive(order)/>
                    </Form>
                </section>

                <section class="grid mb-4 grid-cols-checkbox-8">
                    <AutoListHeader all_items checked_items>
                        "Name"
                        ""
                        "Sets"
                        "Reps"
                        "Sequence"
                        "Workout Plan"
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
                        <BulkDeleteForm
                            table="workout_plan"
                            action=action_bulk_delete
                            checked_items
                        />
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
            </div>

        </main>
    }
}

#[component]
pub fn ExercisePlanListItem<'a>(
    data: &'a ExercisePlanQuery,
    checked_items: RwSignal<HashSet<String>>,
) -> impl IntoView {
    let created_at = format_datetime(&Some(data.created_at));
    let updated_at = format_datetime(&data.updated_at);

    let exercise_id = data.id.to_string();
    let exercise_name = data.movement_name.clone();
    let detail_href = format!(
        "/training-plans/workout-plans/{}/{}",
        data.workout_plan_slug, data.id
    );

    view! {
        <div class="contents group">
            <div class="flex justify-center items-center border-b group-hover:bg-gray-200 group-odd:bg-gray-50">
                <CheckboxListItem id=exercise_id checked_items/>
            </div>
            <div class="flex col-span-2 items-center p-2 border-b group-hover:bg-gray-200 group-odd:bg-gray-50 truncate">
                <A href=detail_href>{exercise_name}</A>
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
