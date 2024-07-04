use std::collections::HashSet;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use super::model::MuscleGroup;
use crate::component::bulk_delete::BulkDeleteForm;
use crate::component::checkbox::CheckboxListItem;
use crate::component::paginator::Paginator;
use crate::component::select::MUSCLE_GROUP_SORT_OPTIONS;
use crate::component::template::{
    AutoListHeader, ErrorComponent, ListNotFoundComponent, ListPageHeaderWithCreate, SearchForm,
    Skeleton,
};
use crate::util::datetime::format_datetime;
use crate::util::misc::ListResponse;
use crate::util::param::{extract_page, extract_param, extract_size};

#[cfg(feature = "ssr")]
use crate::auth::service::get_request_user;

#[server(endpoint = "muscle-group-list")]
pub async fn get_muscle_group_list(
    search: String,
    order: String,
    size: i64,
    page: i64,
) -> Result<ListResponse<MuscleGroup>, ServerFnError> {
    get_request_user()?;
    let pool = expect_context::<sqlx::PgPool>();
    let count = MuscleGroup::count(&pool, &search).await?;
    let results = MuscleGroup::filter(&pool, &search, &order, size, page).await?;
    Ok(ListResponse { count, results })
}

#[component]
pub fn MuscleGroupListPage() -> impl IntoView {
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
        |(search, order, size, page, _)| get_muscle_group_list(search, order, size, page),
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
                    .map(|data| view! { <MuscleGroupListItem data checked_items/> })
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
        <Title text="Muscle Groups"/>
        <main class="md:p-4">

            <div class="p-4 bg-white border">

                <ListPageHeaderWithCreate title="Muscle Groups" create_href="create">
                    <Transition>{count}</Transition>
                </ListPageHeaderWithCreate>

                <section class="flex flex-wrap gap-2 mb-4 lg:mb-2">
                    <SearchForm
                        search=Signal::derive(search)
                        order=Signal::derive(order)
                        size=Signal::derive(size)
                        page=1
                        options=&MUSCLE_GROUP_SORT_OPTIONS
                    />
                </section>

                <section class="grid mb-4 grid-cols-checkbox-4">
                    <AutoListHeader all_items checked_items>
                        "Name"
                        "Exercises"
                        "Created"
                        "Updated"
                    </AutoListHeader>
                    <Transition fallback=|| view! { <Skeleton row_count=25/> }>
                        <ErrorBoundary fallback=|errors| {
                            view! { <ErrorComponent errors/> }
                        }>{response}</ErrorBoundary>
                    </Transition>
                </section>

                <section class="flex flex-wrap">
                    <div>
                        <BulkDeleteForm
                            table="muscle_group"
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
pub fn MuscleGroupListItem<'a>(
    data: &'a MuscleGroup,
    checked_items: RwSignal<HashSet<String>>,
) -> impl IntoView {
    let created_at = format_datetime(&Some(data.created_at));
    let updated_at = format_datetime(&data.updated_at);
    let slug = data.slug.clone();
    let name = data.name.clone();
    view! {
        <div class="contents group">
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
                <CheckboxListItem id=data.id.to_string() checked_items/>
            </div>
            <div class="p-2 group-hover:bg-gray-200 group-odd:bg-gray-50 truncate">
                <A href=slug class="hover:underline">
                    {name}
                </A>
            </div>
            <div class="p-2 group-hover:bg-gray-200 group-odd:bg-gray-50 truncate">
                {data.exercise_count}
            </div>
            <div class="p-2 group-hover:bg-gray-200 group-odd:bg-gray-50 truncate">
                {created_at}
            </div>
            <div class="p-2 group-hover:bg-gray-200 group-odd:bg-gray-50 truncate">
                {updated_at}
            </div>
        </div>
    }
}
