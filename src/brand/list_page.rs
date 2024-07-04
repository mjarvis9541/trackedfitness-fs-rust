use std::collections::HashSet;

use leptos::server_fn::codec::GetUrl;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::brand::model::BrandQuery;
use crate::component::bulk_delete::BulkDeleteForm;
use crate::component::checkbox::CheckboxListItem;
use crate::component::paginator::Paginator;
use crate::component::select::BRAND_SORT_OPTIONS;
use crate::component::template::{
    AutoListHeader, ErrorComponent, ListNotFoundComponent, ListPageHeaderWithCreate, SearchForm,
    Skeleton,
};
use crate::util::datetime::format_datetime;
use crate::util::misc::ListResponse;
use crate::util::param::{extract_page, extract_param, extract_size};

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, setup::get_pool};

#[server(endpoint = "brand-list", input = GetUrl)]
pub async fn get_brand_list(
    search: String,
    order: String,
    size: i64,
    page: i64,
) -> Result<ListResponse<BrandQuery>, ServerFnError> {
    let _user = get_request_user()?;
    let pool = get_pool()?;
    let count = BrandQuery::count(&pool, &search).await?;
    let results = BrandQuery::filter(&pool, &search, &order, size, page).await?;
    Ok(ListResponse { count, results })
}

#[component]
pub fn BrandListPage() -> impl IntoView {
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
        |(search, order, size, page, _)| get_brand_list(search, order, size, page),
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
                        view! { <BrandListItem data=inner.clone() checked_items/> }
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
        <Title text="Brands"/>
        <main class="md:p-4">

            <div class="p-4 bg-white border">
                <ListPageHeaderWithCreate title="Brands" create_href="create">
                    <Transition>{count}</Transition>
                </ListPageHeaderWithCreate>

                <section class="flex flex-wrap gap-2 mb-4 lg:mb-2">
                    <SearchForm
                        search=Signal::derive(search)
                        order=Signal::derive(order)
                        size=Signal::derive(size)
                        page=1
                        options=&BRAND_SORT_OPTIONS
                    />
                </section>

                <section class="grid grid-cols-checkbox-4">
                    <AutoListHeader all_items checked_items>
                        "Name"
                        "Food"
                        "Created"
                        "Updated"
                    </AutoListHeader>
                    <Transition fallback=|| view! { <Skeleton row_count=25/> }>
                        <ErrorBoundary fallback=|errors| {
                            view! { <ErrorComponent errors/> }
                        }>{response}</ErrorBoundary>
                    </Transition>
                </section>

                <section class="flex flex-wrap pt-4">
                    <div class="hidden md:block">
                        <BulkDeleteForm table="food_brand" action=action_bulk_delete checked_items/>
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
pub fn BrandListItem(data: BrandQuery, checked_items: RwSignal<HashSet<String>>) -> impl IntoView {
    let created_at = format_datetime(&Some(data.created_at));
    let updated_at = format_datetime(&data.updated_at);

    let query_url = format!("/food?brand={}", data.slug);
    view! {
        <div class="contents group">
            <div class="flex justify-center items-center group-hover:bg-gray-200 group-odd:bg-gray-50">
                <CheckboxListItem id=data.id.to_string() checked_items/>
            </div>
            <div class="p-2 group-hover:bg-gray-200 group-odd:bg-gray-50 truncate">
                <A href=data.slug class="hover:underline">
                    {data.name}
                </A>
            </div>
            <div class="p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
                <A href=query_url class="hover:underline">
                    {data.food_count}
                </A>
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
