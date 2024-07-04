use leptos::*;
use leptos_router::*;

use crate::component::template::{
    ErrorComponent, ListNotFoundComponent, Loading, LoadingComponent,
};
use crate::follower::component::UserFollowerListItem;
use crate::follower::form::{FollowerUnfollow, FollowerUnfollowForm};
use crate::follower::model::Follower;
use crate::util::datetime::format_datetime;
use crate::util::misc::ListResponse;
use crate::util::param::{extract_page, extract_param, extract_size};

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, follower::status::FollowerStatus};

#[server]
pub async fn get_sent_follower_requests(
    search: String,
    order: String,
    size: i64,
    page: i64,
) -> Result<ListResponse<Follower>, ServerFnError> {
    let token = get_request_user()?;
    let pool = expect_context::<sqlx::PgPool>();
    let follower_status = FollowerStatus::Pending;
    let status: i32 = follower_status.into();
    let count = Follower::get_user_following_count(&pool, &token.username, &search, status).await?;
    let results =
        Follower::get_user_following(&pool, &token.username, &search, status, &order, page, size)
            .await?;
    Ok(ListResponse { count, results })
}

#[component]
pub fn SentFollowerRequestList() -> impl IntoView {
    let query = use_query_map();
    let search = move || extract_param(&query, "search");
    let order = move || extract_param(&query, "order");
    let size = move || extract_size(&query);
    let page = move || extract_page(&query);

    let action_unfollow = expect_context::<Action<FollowerUnfollow, Result<(), ServerFnError>>>();

    let resource = Resource::new(
        move || {
            (
                search(),
                order(),
                size(),
                page(),
                action_unfollow.version().get(),
            )
        },
        |(search, order, size, page, _)| get_sent_follower_requests(search, order, size, page),
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
                        let created_at = format_datetime(&Some(data.created_at));
                        view! {
                            <UserFollowerListItem username=username.clone() created_at>
                                <FollowerUnfollowForm
                                    action=action_unfollow
                                    label="Cancel"
                                    username=username
                                />
                            </UserFollowerListItem>
                        }
                    })
                    .collect_view()
            }
        })
    };
    let count = move || resource.map(|res| res.as_ref().map(|res| res.count).unwrap_or_default());

    view! {
        <header class="mb-4">
            <h2 class="mb-2 text-base font-bold">
                "Sent Requests" <Transition fallback=Loading>" (" {count} ")"</Transition>
            </h2>
            <p>"Users that you have requested to follow."</p>
        </header>
        <Transition fallback=LoadingComponent>
            <ErrorBoundary fallback=|errors| {
                view! { <ErrorComponent errors/> }
            }>{response}</ErrorBoundary>
        </Transition>
    }
}
