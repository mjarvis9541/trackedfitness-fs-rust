use std::collections::HashSet;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use super::follower_create_form::AdminFollowerCreateForm;
use super::user_select::{get_user_select, UserSelectResource};
use crate::component::bulk_delete::BulkDeleteForm;
use crate::component::checkbox::CheckboxListItem;
use crate::component::input::FilterInput;
use crate::component::paginator::Paginator;
use crate::component::select::FilterSelect;
use crate::component::template::{
    AutoListHeader, ErrorComponent, ListLoadingComponent, ListNotFoundComponent, Loading,
};
use crate::follower::model::Follower;
use crate::util::datetime::format_datetime;
use crate::util::misc::ListResponse;
use crate::util::param::{extract_page, extract_param, extract_size};

#[server(endpoint = "get-admin-follower-list")]
pub async fn get_admin_follower_list(
    username: String,
    follower: String,
    status: String,
    order: String,
    size: i64,
    page: i64,
) -> Result<ListResponse<Follower>, ServerFnError> {
    crate::auth::service::extract_superuser_from_request()?;
    let pool = crate::setup::get_pool()?;

    let count = Follower::count(&pool, &username, &follower, &status).await?;
    let results =
        Follower::filter(&pool, &username, &follower, &status, &order, size, page).await?;
    Ok(ListResponse { count, results })
}

#[component]
pub fn AdminFollowerListPage() -> impl IntoView {
    let resource: UserSelectResource = Resource::once(get_user_select);
    provide_context(resource);

    let action_create = Action::server();
    let action_bulk_delete = Action::server();

    let query = use_query_map();
    let username = move || extract_param(&query, "username");
    let follower = move || extract_param(&query, "follower");
    let status = move || extract_param(&query, "status");
    let order = move || extract_param(&query, "order");
    let size = move || extract_size(&query);
    let page = move || extract_page(&query);

    let follower_resource = Resource::new(
        move || {
            (
                username(),
                follower(),
                status(),
                order(),
                size(),
                page(),
                action_create.version().get(),
                action_bulk_delete.version().get(),
            )
        },
        |(username, follower, status, order, size, page, _, _)| {
            get_admin_follower_list(username, follower, status, order, size, page)
        },
    );

    let all_items = RwSignal::new(HashSet::new());
    let checked_items = RwSignal::new(HashSet::new());

    let response = move || {
        follower_resource.and_then(|data| {
            let count = &data.count;
            let results = &data.results;
            if *count == 0 {
                view! { <ListNotFoundComponent/> }
            } else {
                let ids: HashSet<String> = results.iter().map(|item| item.id.to_string()).collect();
                all_items.update(|set| set.extend(ids));
                results
                    .iter()
                    .map(|data| {
                        view! { <AdminFollowerListItem data=data.clone() checked_items/> }
                    })
                    .collect_view()
            }
        })
    };

    let count = move || {
        follower_resource.with(|opt| {
            opt.as_ref()
                .and_then(|res| res.as_ref().map(|res| res.count).ok())
        })
    };

    view! {
        <Title text="Admin - User Followers"/>
        <main class="lg:p-4">

            <div class="grid grid-cols-4 gap-4 lg:grid-cols-12">

                <div class="col-span-4 p-4 bg-white lg:col-span-9">

                    <header class="mb-2">
                        <h2 class="text-base font-bold">"Admin - User Followers"</h2>
                        <p class="text-gray-400">
                            "Results: " <Transition fallback=Loading>{count}</Transition>
                        </p>
                    </header>

                    <section class="flex flex-wrap gap-2 mb-2 whitespace-nowrap">
                        <Form method="GET" action="" class="contents">
                            <input type="hidden" name="size" value=size/>
                            <input type="hidden" name="page" value=1/>
                            <FilterInput name="username" value=Signal::derive(username)/>
                            <FilterInput name="follower" value=Signal::derive(follower)/>
                            <FilterSelect
                                name="status"
                                value=Signal::derive(status)
                                options=&crate::component::select::FOLLOWER_STATUS_OPTIONS
                            />
                            <FilterSelect
                                name="order"
                                value=Signal::derive(order)
                                options=&crate::component::select::FOLLOWER_SORT_OPTIONS
                            />
                        </Form>
                    </section>

                    <section class="grid overflow-auto mb-4 whitespace-nowrap grid-cols-checkbox-6">
                        <AutoListHeader all_items checked_items>
                            "Username"
                            "Follower"
                            "Status"
                            "Created"
                            "Updated"
                            "Edit"
                        </AutoListHeader>
                        <Transition fallback=ListLoadingComponent>
                            <ErrorBoundary fallback=|errors| {
                                view! { <ErrorComponent errors=errors/> }
                            }>{response}</ErrorBoundary>
                        </Transition>
                    </section>

                    <section class="flex flex-wrap">
                        <BulkDeleteForm
                            table="user_follower"
                            action=action_bulk_delete
                            checked_items
                        />
                        <div class="flex-1">
                            <Form method="GET" action="" class="contents">
                                <input type="hidden" name="username" value=username/>
                                <input type="hidden" name="follower" value=follower/>
                                <input type="hidden" name="status" value=status/>
                                <input type="hidden" name="order" value=order/>
                                <input type="hidden" name="page" value=page/>
                                <Transition>
                                    <Paginator count/>
                                </Transition>
                            </Form>
                        </div>
                    </section>
                </div>

                <section class="col-span-4 lg:col-span-3">
                    <div class="p-4 mb-4 bg-white">
                        <h2 class="mb-2 text-base font-bold">"Create Follower"</h2>
                        <AdminFollowerCreateForm action=action_create/>
                    </div>
                </section>
            </div>
        </main>
    }
}

#[component]
fn AdminFollowerListItem(
    data: Follower,
    checked_items: RwSignal<HashSet<String>>,
) -> impl IntoView {
    let created_at = format_datetime(&Some(data.created_at));
    let updated_at = format_datetime(&data.updated_at);

    view! {
        <div class="contents group">
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
                <CheckboxListItem id=data.id.to_string() checked_items/>
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
                <A href=format!("/users/{}", &data.username) class="text-blue-500 hover:underline">
                    {data.username}
                </A>
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
                <A href=format!("/users/{}", &data.follower) class="text-blue-500 hover:underline">
                    {data.follower}
                </A>
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
                {data.status.to_string()}
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
                {created_at}
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
                {updated_at}
            </div>
            <div class="flex justify-center items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
                <A
                    href=format!("/admin/followers/{}", &data.id)
                    class="text-blue-500 hover:underline"
                >
                    "Edit"
                </A>
            </div>
        </div>
    }
}
