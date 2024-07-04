use std::collections::HashSet;

use leptos::server_fn::codec::GetUrl;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::component::bulk_delete::BulkDeleteForm;
use crate::component::checkbox::CheckboxListItem;
use crate::component::paginator::Paginator;
use crate::component::select::DATE_SORT_OPTIONS;
use crate::component::template::{
    AutoListHeader, ErrorComponent, ListLoadingComponent, ListNotFoundComponent, Loading,
    SearchForm,
};
use crate::progress::create_page::ProgressCreate;
use crate::progress::delete_page::ProgressDelete;
use crate::progress::model::Progress;
use crate::progress::update_page::ProgressUpdate;
use crate::util::param::{extract_page, extract_param, extract_size, UsernameParam};

#[cfg(feature = "ssr")]
use crate::{auth::model::User, auth::service::get_request_user, setup::get_pool};

#[server(endpoint = "progress-list", input = GetUrl)]
pub async fn get_progress_list(
    username: String,
    search: String,
    order: String,
    size: i64,
    page: i64,
) -> Result<Vec<Progress>, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    User::check_view_permission(&pool, &user, &username).await?;
    let results = Progress::filter(&pool, &search, &username, &order, size, page).await?;
    Ok(results)
}

#[server(endpoint = "progress-list-count", input = GetUrl)]
pub async fn get_progress_list_count(
    username: String,
    search: String,
) -> Result<i64, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    User::check_view_permission(&pool, &user, &username).await?;
    let count = Progress::count(&pool, &search, &username).await?;
    Ok(count)
}

#[component]
pub fn ProgressListPage() -> impl IntoView {
    let action_bulk_delete = Action::server();
    let action_create = Action::<ProgressCreate, _>::server();
    let action_update = Action::<ProgressUpdate, _>::server();
    let action_delete = Action::<ProgressDelete, _>::server();

    provide_context(action_create);
    provide_context(action_update);
    provide_context(action_delete);

    let params = use_params::<UsernameParam>();
    let username =
        move || params.with(|p| p.as_ref().map(|p| p.username.clone()).unwrap_or_default());

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
                action_create.version().get(),
                action_update.version().get(),
                action_delete.version().get(),
                action_bulk_delete.version().get(),
            )
        },
        |(username, search, order, size, page, ..)| {
            get_progress_list(username, search, order, size, page)
        },
    );
    let count_resource = Resource::new(
        move || {
            (
                username(),
                search(),
                action_create.version().get(),
                action_update.version().get(),
                action_delete.version().get(),
                action_bulk_delete.version().get(),
            )
        },
        |(username, search, _, _, _, _)| get_progress_list_count(username, search),
    );

    let all_items = RwSignal::new(HashSet::<String>::new());
    let checked_items = RwSignal::new(HashSet::<String>::new());

    let response = move || {
        resource.and_then(|data| {
            if data.is_empty() {
                view! { <ListNotFoundComponent/> }
            } else {
                let ids: HashSet<String> = data.iter().map(|item| item.id.to_string()).collect();
                all_items.update(|set| set.extend(ids));
                data.iter()
                    .map(|data| view! { <ProgressListItem data checked_items/> })
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

    view! {
        <Title text="Progress History"/>
        <main class="m-4 p-4 border bg-white">

            <header class="flex justify-between mb-2">
                <div>
                    <h1 class="text-xl font-bold">"Progress History"</h1>
                    <p class="text-gray-400">
                        "Results: " <Transition fallback=Loading>{count}</Transition>
                    </p>
                </div>
                <div class="flex">
                    <div class=(
                        "hidden",
                        move || checked_item_count() == 0,
                    )>{checked_item_count} " selected"</div>
                </div>
            </header>

            <section class="flex flex-wrap gap-2 mb-4 lg:mb-2">
                <SearchForm
                    search=Signal::derive(search)
                    order=Signal::derive(order)
                    size=Signal::derive(size)
                    page=1
                    options=&DATE_SORT_OPTIONS
                />
            </section>

            <section class="grid overflow-auto mb-4 grid-cols-checkbox-9">
                <AutoListHeader all_items checked_items>
                    "Date"
                    "Day"
                    "Weight"
                    "Week Avg"
                    "Month Avg"
                    "Energy Burnt"
                    "Week Avg"
                    "Month Avg"
                    "Notes"
                </AutoListHeader>
                <Transition fallback=ListLoadingComponent>
                    <ErrorBoundary fallback=|errors| {
                        view! { <ErrorComponent errors/> }
                    }>{response}</ErrorBoundary>
                </Transition>
            </section>

            <section class="flex flex-wrap">
                <div>
                    <BulkDeleteForm table="progress" action=action_bulk_delete checked_items/>
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

#[component]
pub fn ProgressListItem<'a>(
    data: &'a Progress,
    checked_items: RwSignal<HashSet<String>>,
) -> impl IntoView {
    let date = data.date.to_string();

    let date_display = data.date.format("%d/%m/%Y").to_string();
    let day_display = data.date.format("%A").to_string();

    view! {
        <div class="contents group">
            <div class="p-2 font-bold border-b group-hover:bg-amber-200 group-odd:bg-gray-50">
                <CheckboxListItem id=data.id.to_string() checked_items/>
            </div>
            <div class="flex border-b group-hover:bg-amber-200 group-odd:bg-gray-50 truncate">
                <A
                    href=date.clone()
                    class="flex flex-1 p-2 aria-[current=page]:bg-amber-200"
                    exact=true
                >
                    {date_display}
                </A>
            </div>
            <div class="flex border-b group-hover:bg-amber-200 group-odd:bg-gray-50 truncate">
                <A href=date class="flex flex-1 p-2 aria-[current=page]:bg-amber-200" exact=true>
                    {day_display}
                </A>
            </div>
            <div class="p-2 border-b group-hover:bg-amber-200 group-odd:bg-gray-50">
                {data.weight.map_or_else(|| "-".to_string(), |x| format!("{:.2}kg", x))}
            </div>
            <div class="p-2 border-b group-hover:bg-amber-200 group-odd:bg-gray-50">
                {data.week_avg_weight.map_or_else(|| "-".to_string(), |x| format!("{:.2}kg", x))}
            </div>
            <div class="p-2 border-b group-hover:bg-amber-200 group-odd:bg-gray-50">
                {data.month_avg_weight.map_or_else(|| "-".to_string(), |x| format!("{:.2}kg", x))}
            </div>
            <div class="p-2 border-b group-hover:bg-amber-200 group-odd:bg-gray-50">
                {data.energy_burnt.map_or_else(|| "-".to_string(), |d| d.to_string())}
            </div>
            <div class="p-2 border-b group-hover:bg-amber-200 group-odd:bg-gray-50">
                {data.week_avg_energy_burnt.map_or_else(|| "-".to_string(), |d| d.to_string())}
            </div>
            <div class="p-2 border-b group-hover:bg-amber-200 group-odd:bg-gray-50">
                {data.month_avg_energy_burnt.map_or_else(|| "-".to_string(), |d| d.to_string())}
            </div>
            <div class="p-2 border-b group-hover:bg-amber-200 group-odd:bg-gray-50 truncate">
                {data.notes.as_ref().map(|d| d).unwrap_or(&"-".to_string())}
            </div>
        </div>
    }
}
