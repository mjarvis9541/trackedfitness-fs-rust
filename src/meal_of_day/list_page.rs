use std::collections::HashSet;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use super::model::MealOfDay;
use crate::component::bulk_delete::BulkDeleteForm;
use crate::component::checkbox::CheckboxListItem;
use crate::component::input::FilterInput;
use crate::component::paginator::Paginator;
use crate::component::select::FilterSelect;
use crate::component::template::{
    AutoListHeader, ErrorComponent, ListLoadingComponent, ListNotFoundComponent,
    ListPageHeaderWithCreate, Loading,
};
use crate::util::datetime::format_datetime;
use crate::util::misc::ListResponse;
use crate::util::param::{extract_page, extract_param, extract_size};

#[cfg(feature = "ssr")]
use crate::auth::service::get_request_user;

#[server(endpoint = "meal-of-day-list")]
pub async fn get_meal_of_day_list(
    search: String,
    order: String,
    size: i64,
    page: i64,
) -> Result<ListResponse<MealOfDay>, ServerFnError> {
    get_request_user()?;
    let pool = expect_context::<sqlx::PgPool>();
    let count = MealOfDay::count(&pool, &search).await?;
    let results = MealOfDay::filter(&pool, &search, &order, size, page).await?;
    Ok(ListResponse { count, results })
}

#[component]
pub fn MealOfDayListPage() -> impl IntoView {
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
        |(search, order, size, page, _)| get_meal_of_day_list(search, order, size, page),
    );
    let all_items = RwSignal::new(HashSet::new());
    let checked_items = RwSignal::new(HashSet::new());

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
                    .map(|data| view! { <MealOfDayListItem data checked_items/> })
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
    let sort_options = vec![
        ("name", "Name (A-z)"),
        ("-name", "Name (Z-a)"),
        ("ordering", "Order (Asc)"),
        ("-ordering", "Order (Desc)"),
        ("created_at", "Created (Asc)"),
        ("-created_at", "Created (Desc)"),
        ("updated_at", "Updated (Asc)"),
        ("-updated_at", "Updated (Desc)"),
    ];

    view! {
        <Title text="Meal of Day"/>
        <main class="md:p-4">

            <div class="p-4 bg-white border">

                <ListPageHeaderWithCreate title="Meal of Day" create_href="create">
                    <Transition fallback=Loading>{count}</Transition>
                </ListPageHeaderWithCreate>

                <section class="flex flex-wrap gap-x-2 mb-2">
                    <Form method="GET" action="" class="contents">
                        <input type="hidden" name="page" value=1/>
                        <input type="hidden" name="size" value=size/>
                        <FilterInput name="search" value=Signal::derive(search)/>
                        <FilterSelect
                            name="order"
                            value=Signal::derive(order)
                            options=sort_options
                        />
                    </Form>
                </section>

                <div class="grid grid-cols-checkbox-4">
                    <AutoListHeader all_items checked_items>
                        "Name"
                        "Order"
                        "Created"
                        "Updated"
                    </AutoListHeader>
                    <Transition fallback=ListLoadingComponent>
                        <ErrorBoundary fallback=|errors| {
                            view! { <ErrorComponent errors/> }
                        }>{response}</ErrorBoundary>
                    </Transition>
                </div>

                <div class="flex flex-wrap pt-4">
                    <div>
                        <BulkDeleteForm
                            table="meal_of_day"
                            action=action_bulk_delete
                            checked_items
                        />
                    </div>

                    <div class="flex-1">
                        <Form method="GET" action="" class="contents">
                            <input type="hidden" name="page" value=page/>
                            <input type="hidden" name="search" value=search/>
                            <input type="hidden" name="order" value=order/>
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

#[component]
pub fn MealOfDayListItem<'a>(
    data: &'a MealOfDay,
    checked_items: RwSignal<HashSet<String>>,
) -> impl IntoView {
    let created_at = format_datetime(&Some(data.created_at));
    let updated_at = format_datetime(&data.updated_at);
    let detail_href = data.slug.clone();
    let name = data.name.clone();
    view! {
        <div class="contents group">
            <div class="flex justify-center items-center group-hover:bg-gray-200 group-odd:bg-gray-50">
                <CheckboxListItem id=data.id.to_string() checked_items/>
            </div>
            <div class="p-2 group-hover:bg-gray-200 group-odd:bg-gray-50 truncate">
                <A href=detail_href>{name}</A>
            </div>
            <div class="p-2 group-hover:bg-gray-200 group-odd:bg-gray-50 truncate">
                {data.ordering}
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
