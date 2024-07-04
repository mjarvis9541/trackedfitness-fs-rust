use std::collections::HashSet;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use super::user_block_create_form::AdminUserBlockCreateForm;
use crate::component::bulk_delete::BulkDeleteForm;
use crate::component::checkbox::CheckboxListItem;
use crate::component::input::FilterInput;
use crate::component::paginator::Paginator;
use crate::component::select::FilterSelect;
use crate::component::template::{
    AutoListHeader, ErrorComponent, ListLoadingComponent, ListNotFoundComponent, Loading,
};
use crate::user_block::model::UserBlock;
use crate::util::datetime::format_datetime;
use crate::util::misc::ListResponse;
use crate::util::param::{extract_page, extract_param, extract_size};

#[server]
pub async fn get_admin_user_block_list(
    blocker: String,
    blocked: String,
    status: String,
    order: String,
    size: i64,
    page: i64,
) -> Result<ListResponse<UserBlock>, ServerFnError> {
    crate::auth::service::extract_superuser_from_request()?;
    let pool = crate::setup::get_pool()?;
    let count = UserBlock::count(&pool, &blocker, &blocked, &status).await?;
    let results = UserBlock::filter(&pool, &blocker, &blocked, &status, &order, size, page).await?;
    Ok(ListResponse { count, results })
}

#[component]
pub fn AdminUserBlockListPage() -> impl IntoView {
    let action_block = Action::server();
    let action_bulk_delete = Action::server();

    let query = use_query_map();
    let blocker = move || extract_param(&query, "blocker");
    let blocked = move || extract_param(&query, "blocked");
    let status = move || extract_param(&query, "status");
    let order = move || extract_param(&query, "order");
    let size = move || extract_size(&query);
    let page = move || extract_page(&query);

    let resource = Resource::new(
        move || {
            (
                blocker(),
                blocked(),
                status(),
                order(),
                size(),
                page(),
                action_block.version().get(),
                action_bulk_delete.version().get(),
            )
        },
        |(blocker, blocked, status, order, size, page, ..)| {
            get_admin_user_block_list(blocker, blocked, status, order, size, page)
        },
    );

    let all_items = RwSignal::new(HashSet::<String>::new());
    let checked_items = RwSignal::new(HashSet::<String>::new());

    let response = move || {
        resource.and_then(|data| {
            if data.results.is_empty() {
                view! { <ListNotFoundComponent/> }
            } else {
                data.results
                    .iter()
                    .map(|data| {
                        all_items.update(|v| {
                            v.insert(data.id.to_string());
                        });
                        view! { <AdminUserBlockListItem data=data.clone() checked_items/> }
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
        <Title text="Admin - Blocked Users"/>
        <main class="lg:p-4">

            <div class="grid grid-cols-4 gap-4 lg:grid-cols-12">
                <div class="col-span-4 lg:col-span-9">

                    <div class="p-4 mb-2 bg-white">
                        <header class="mb-2">
                            <h2 class="text-base font-bold">"Admin - Blocked Users"</h2>
                            <p class="text-gray-400">
                                "Results: " <Transition fallback=Loading>{count}</Transition>
                            </p>
                        </header>

                        <div class="flex flex-wrap gap-2 mb-2 whitespace-nowrap">
                            <Form method="GET" action="" class="contents">
                                <input type="hidden" name="page" value=1/>
                                <input type="hidden" name="size" value=size/>
                                <FilterInput name="blocker" value=Signal::derive(blocker)/>
                                <FilterInput name="blocked" value=Signal::derive(blocked)/>
                                <FilterSelect
                                    name="status"
                                    value=Signal::derive(status)
                                    options=&crate::component::select::BLOCKED_STATUS_OPTIONS
                                />
                                <FilterSelect
                                    name="order"
                                    value=Signal::derive(order)
                                    options=&crate::component::select::USER_SORT_OPTIONS
                                />
                            </Form>
                        </div>
                    </div>

                    <section class="grid overflow-auto p-4 whitespace-nowrap bg-white grid-cols-checkbox-6">
                        <AutoListHeader all_items checked_items>
                            "Blocker"
                            "Blocked"
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

                    <div class="flex flex-wrap justify-between p-4 bg-white">

                        <BulkDeleteForm table="user_block" action=action_bulk_delete checked_items/>

                        <div>
                            <Form method="GET" action="" class="contents">
                                <input type="hidden" name="blocker" value=blocker/>
                                <input type="hidden" name="blocked" value=blocked/>
                                <input type="hidden" name="status" value=status/>
                                <input type="hidden" name="page" value=page/>
                                <input type="hidden" name="order" value=order/>
                                <Transition>
                                    <Paginator count/>
                                </Transition>
                            </Form>
                        </div>

                    </div>
                </div>

                <section class="col-span-4 lg:col-span-3">
                    <div class="p-4 mb-4 bg-white">
                        <h2 class="mb-2 text-base font-bold">"Create User Block"</h2>
                        <AdminUserBlockCreateForm action=action_block/>
                    </div>

                </section>
            </div>
        </main>
    }
}

#[component]
fn AdminUserBlockListItem(
    data: UserBlock,
    checked_items: RwSignal<HashSet<String>>,
) -> impl IntoView {
    let created_at = format_datetime(&Some(data.blocked_at));
    let updated_at = format_datetime(&data.unblocked_at);

    view! {
        <div class="contents group">
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
                <CheckboxListItem id=data.id.to_string() checked_items/>
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
                <A
                    href=format!("/users/{}", &data.blocker_username)
                    class="text-blue-500 hover:underline"
                >
                    {data.blocker_username}
                </A>
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
                <A
                    href=format!("/users/{}", &data.blocked_username)
                    class="text-blue-500 hover:underline"
                >
                    {data.blocked_username}
                </A>
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
                {data.blocked_status.to_string()}
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
                {created_at}
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
                {updated_at}
            </div>
            <div class="flex justify-center items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
                <A
                    href=format!("/admin/blocked-users/{}", &data.id)
                    class="text-blue-500 hover:underline"
                >
                    "Edit"
                </A>
            </div>
        </div>
    }
}
