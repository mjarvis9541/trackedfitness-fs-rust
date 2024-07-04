use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use super::model::Follower;
use crate::component::paginator::Paginator;
use crate::component::select::USER_SORT_OPTIONS;
use crate::component::template::{
    ErrorComponent, ListNotFoundComponent, LoadingComponent, SearchForm,
};
use crate::follower::component::FollowerListItem;
use crate::util::misc::ListResponse;
use crate::util::param::{extract_page, extract_param, extract_size, get_username};

#[cfg(feature = "ssr")]
use crate::auth::service::get_request_user;

#[server(endpoint = "user-following-list")]
pub async fn get_user_following_list(
    username: String,
    search: String,
    order: String,
    size: i64,
    page: i64,
) -> Result<ListResponse<Follower>, ServerFnError> {
    get_request_user()?;
    let pool = expect_context::<sqlx::PgPool>();
    let follower_status = crate::follower::status::FollowerStatus::Accepted;
    let status: i32 = follower_status.into();
    let count = Follower::get_user_following_count(&pool, &username, &search, status).await?;
    let results =
        Follower::get_user_following(&pool, &username, &search, status, &order, page, size).await?;
    Ok(ListResponse { count, results })
}

#[component]
pub fn UserFollowingPage() -> impl IntoView {
    let params = use_params_map();
    let username = move || get_username(&params);

    let query = use_query_map();
    let search = move || extract_param(&query, "search");
    let order = move || extract_param(&query, "order");
    let size = move || extract_size(&query);
    let page = move || extract_page(&query);

    let resource = Resource::new(
        move || (username(), search(), order(), size(), page()),
        |(username, search, order, size, page)| {
            get_user_following_list(username, search, order, size, page)
        },
    );
    let response = move || {
        resource.and_then(|data| {
            let results = &data.results;
            if results.is_empty() {
                view! { <ListNotFoundComponent/> }
            } else {
                results
                    .iter()
                    .map(|data| {
                        let username = data.username.clone();
                        view! { <FollowerListItem username/> }
                    })
                    .collect_view()
            }
        })
    };
    let count = move || resource.map(|res| res.as_ref().map(|res| res.count).unwrap_or_default());

    view! {
        <Title text="Following"/>
        <main class="p-4 m-4 bg-white border">
            <header class="mb-4">
                <h1 class="text-xl font-bold">"Following"</h1>
                <p class="text-gray-500">"Results: " <Transition>{count}</Transition></p>
            </header>
            <section class="flex flex-wrap gap-2 mb-4 lg:mb-2">
                <SearchForm
                    search=Signal::derive(search)
                    order=Signal::derive(order)
                    size=Signal::derive(size)
                    page=1
                    options=&USER_SORT_OPTIONS
                />
            </section>
            <section class="my-4">
                <Transition fallback=LoadingComponent>
                    <ErrorBoundary fallback=|errors| {
                        view! { <ErrorComponent errors/> }
                    }>{response}</ErrorBoundary>
                </Transition>
            </section>
            <section class="flex-1">
                <Form method="GET" action="" class="contents">
                    <input type="hidden" name="search" value=search/>
                    <input type="hidden" name="order" value=order/>
                    <input type="hidden" name="page" value=page/>
                    <Transition>
                        <Paginator count/>
                    </Transition>
                </Form>
            </section>
        </main>
    }
}
