use std::collections::HashSet;

use leptos::server_fn::codec::GetUrl;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use super::model::Movement;
use crate::component::bulk_delete::BulkDeleteForm;
use crate::component::input::FilterInput;
use crate::component::paginator::Paginator;
use crate::component::select::{FilterSelect, MOVEMENT_SORT_OPTIONS};
use crate::component::template::{
    AutoListHeader, AutoListItem, ErrorComponent, ListNotFoundComponent, ListPageHeaderWithCreate,
    Skeleton,
};
use crate::muscle_group::select::{get_muscle_group_filter, MuscleGroupFilter};
use crate::util::datetime::format_datetime;
use crate::util::misc::ListResponse;
use crate::util::param::{extract_page, extract_param, extract_size};

#[cfg(feature = "ssr")]
use crate::auth::service::get_request_user;

#[server(endpoint = "movement-list", input = GetUrl)]
pub async fn get_movement_list(
    search: String,
    muscle_group: String,
    order: String,
    size: i64,
    page: i64,
) -> Result<ListResponse<Movement>, ServerFnError> {
    get_request_user()?;
    let pool = expect_context::<sqlx::PgPool>();
    let count = Movement::count(&pool, &search, &muscle_group).await?;
    let results = Movement::filter(&pool, &search, &muscle_group, &order, size, page).await?;
    Ok(ListResponse { count, results })
}

#[component]
pub fn MovementListPage() -> impl IntoView {
    let action_bulk_delete = Action::server();

    let query = use_query_map();
    let search = move || extract_param(&query, "search");
    let muscle_group = move || extract_param(&query, "muscle_group");
    let order = move || extract_param(&query, "order");
    let size = move || extract_size(&query);
    let page = move || extract_page(&query);

    let resource = Resource::new(
        move || {
            (
                search(),
                muscle_group(),
                order(),
                size(),
                page(),
                action_bulk_delete.version().get(),
            )
        },
        |(search, muscle_group, order, size, page, _)| {
            get_movement_list(search, muscle_group, order, size, page)
        },
    );
    let muscle_group_filter = Resource::once(get_muscle_group_filter);
    provide_context(muscle_group_filter);

    let all_items = RwSignal::new(HashSet::<String>::new());
    let checked_items = RwSignal::new(HashSet::<String>::new());

    let response = move || {
        resource.and_then(|data| {
            let count = data.count;
            let results = &data.results;
            if count == 0 {
                view! { <ListNotFoundComponent/> }
            } else {
                let ids: HashSet<String> = results.iter().map(|item| item.id.to_string()).collect();
                all_items.update(|set| set.extend(ids));
                results
                    .iter()
                    .map(|data| {
                        let id = data.id.to_string();
                        let slug = data.slug.clone();
                        let name = data.name.clone();
                        let muscle_group_href =
                            format!("/exercises/muscle-groups/{}", data.muscle_group_slug);
                        let muscle_group_name = data.muscle_group_name.clone();
                        let created = format_datetime(&Some(data.created_at));
                        let updated = format_datetime(&data.updated_at);
                        view! {
                            <AutoListItem id checked_items>
                                <A href=slug class="hover:underline">
                                    {name}
                                </A>
                                <A href=muscle_group_href class="hover:underline">
                                    {muscle_group_name}
                                </A>
                                {created}
                                {updated}
                            </AutoListItem>
                        }
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
        <Title text="Exercises"/>
        <main class="md:p-4">
            <div class="p-4 bg-white border">

                <ListPageHeaderWithCreate title="Exercises" create_href="create">
                    <Transition>{count}</Transition>
                </ListPageHeaderWithCreate>

                <section class="flex flex-wrap gap-2 mb-4 lg:mb-2">

                    <Form method="GET" action="" class="contents">
                        <FilterInput name="search" value=Signal::derive(search)/>
                        <FilterSelect
                            name="order"
                            value=Signal::derive(order)
                            options=&MOVEMENT_SORT_OPTIONS
                        />
                        <MuscleGroupFilter selected=Signal::derive(muscle_group)/>
                        <input type="hidden" name="size" value=size/>
                        <input type="hidden" name="page" value=page/>

                    </Form>
                </section>

                <section class="grid grid-cols-checkbox-4">
                    <AutoListHeader all_items checked_items>
                        "Name"
                        "Mucle Group"
                        "Created"
                        "Updated"
                    </AutoListHeader>

                    <Transition fallback=|| view! { <Skeleton row_count=25/> }>
                        <ErrorBoundary fallback=|errors| {
                            view! { <ErrorComponent errors/> }
                        }>{response}</ErrorBoundary>
                    </Transition>
                </section>

                <div class="flex flex-wrap pt-4">
                    <div>
                        <BulkDeleteForm table="movement" action=action_bulk_delete checked_items/>
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
                </div>
            </div>

        </main>
    }
}
