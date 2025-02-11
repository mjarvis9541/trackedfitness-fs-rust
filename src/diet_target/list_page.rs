use std::collections::HashSet;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::component::bulk_delete_date::BulkDeleteDateRangeForm;
use crate::component::input::FilterInput;
use crate::component::paginator::Paginator;
use crate::component::select::FilterSelect;
use crate::component::template::{
    AutoListHeader, ErrorComponent, ListLoadingComponent, ListNotFoundComponent, Loading,
};
use crate::summary::component::DaySummaryListItem;
use crate::summary::model::{UserDaySummary, Variant};
use crate::util::param::{extract_page, extract_param, extract_size, get_username};

#[cfg(feature = "ssr")]
use crate::{
    auth::model::User, auth::service::get_request_user, diet_target::model::DietTargetQuery,
    setup::get_pool,
};

#[server(endpoint = "diet-target-list")]
pub async fn get_diet_target_list(
    username: String,
    search: String,
    order: String,
    size: i64,
    page: i64,
) -> Result<Vec<UserDaySummary>, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    User::check_view_permission(&pool, &user, &username).await?;

    let results =
        DietTargetQuery::filter_by_username(&pool, &username, &search, &order, size, page).await?;

    Ok(results)
}

#[server(endpoint = "diet-target-list-count")]
pub async fn get_diet_target_list_count(
    username: String,
    search: String,
) -> Result<i64, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    User::check_view_permission(&pool, &user, &username).await?;

    let count = DietTargetQuery::count_by_username(&pool, &username, &search).await?;

    Ok(count)
}

#[component]
pub fn DietTargetListPage() -> impl IntoView {
    let action_bulk_delete = Action::server();

    let params = use_params_map();
    let username = move || get_username(&params);

    let query = use_query_map();
    let search = move || extract_param(&query, "search");
    let order = move || extract_param(&query, "order");
    let size = move || extract_size(&query);
    let page = move || extract_page(&query);
    let resource = Resource::new(
        move || {
            (
                username(),
                search(),
                order(),
                size(),
                page(),
                action_bulk_delete.version().get(),
            )
        },
        |(username, search, order, size, page, ..)| {
            get_diet_target_list(username, search, order, size, page)
        },
    );
    let count_resource = Resource::new(
        move || (username(), search(), action_bulk_delete.version().get()),
        |(username, search, ..)| get_diet_target_list_count(username, search),
    );

    let all_items = RwSignal::new(HashSet::<String>::new());
    let checked_items = RwSignal::new(HashSet::<String>::new());

    let response = move || {
        resource.and_then(|data| {
            if data.is_empty() {
                view! { <ListNotFoundComponent/> }
            } else {
                let ids: HashSet<String> = data.iter().map(|item| item.date.to_string()).collect();
                all_items.update(|set| set.extend(ids));
                data.iter()
                    .map(|data| {
                        view! { <DaySummaryListItem data=data.clone() checked_items variant=Variant::DietTarget/> }
                    })
                    .collect_view()
            }
        })
    };
    let count = move || {
        count_resource.with(|res| {
            res.as_ref()
                .and_then(|data| data.as_ref().ok().map(|res| *res))
        })
    };
    let checked_item_count = move || checked_items.with(|items| items.len());
    let sort_options = vec![
        ("-date", "Date (Desc)"),
        ("date", "Date (Asc)"),
        ("created_at", "Created (Asc)"),
        ("-created_at", "Created (Desc)"),
        ("updated_at", "Updated (Asc)"),
        ("-updated_at", "Updated (Desc)"),
    ];
    view! {
        <Title text="Diet Target History"/>
        <main class="m-4 p-4 border bg-white">
            <header class="flex justify-between mb-2">
                <div>
                    <h1 class="text-xl font-bold">"Diet Target History"</h1>
                    <p class="text-gray-400">
                        "Results: " <Transition fallback=Loading>{count}</Transition>
                    </p>
                </div>
                <div class=(
                    "hidden",
                    move || checked_item_count() == 0,
                )>{checked_item_count} " selected"</div>
            </header>

            <section class="flex flex-wrap gap-2 mb-4 lg:mb-2">
                <Form method="GET" action="" class="contents">
                    <input type="hidden" name="size" value=size/>
                    <input type="hidden" name="page" value=1/>
                    <FilterInput
                        name="search"
                        value=Signal::derive(search)
                        placeholder="Search year YYYYY"
                    />
                    <FilterSelect name="order" value=Signal::derive(order) options=sort_options/>
                </Form>
            </section>
            <div class="overflow-x-auto mb-4">
                <div class="grid grid-cols-checkbox-16">
                    <AutoListHeader all_items checked_items align_right=true>
                        "Date"
                        "Day"
                        "Calories"
                        "Protein"
                        "Carbs"
                        "Fat"
                        "Sat. Fat"
                        "Sugars"
                        "Fibre"
                        "Salt"
                        "Cals/kg"
                        "Pro/kg"
                        "Carbs/kg"
                        "Fat/kg"
                        "Weight"
                    </AutoListHeader>
                    <Transition fallback=ListLoadingComponent>
                        <ErrorBoundary fallback=|errors| {
                            view! { <ErrorComponent errors/> }
                        }>{response}</ErrorBoundary>
                    </Transition>
                </div>
            </div>
            <section class="flex flex-wrap">
                <div>
                    <BulkDeleteDateRangeForm
                        table="diet_target"
                        action=action_bulk_delete
                        checked_items
                        username=Signal::derive(username)
                    />
                </div>
                <div class="flex-1">
                    <Form method="GET" action="" class="contents">
                        <input type="hidden" name="search" value=search/>
                        <input type="hidden" name="page" value=page/>
                        <input type="hidden" name="order" value=order/>
                        <Transition>
                            <Paginator count/>
                        </Transition>
                    </Form>
                </div>
            </section>
        </main>
    }
}
