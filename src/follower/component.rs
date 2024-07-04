use leptos::*;
use leptos_router::*;

use crate::component::icon::IconUsers;

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, follower::model::Follower, setup::get_pool};

#[server(endpoint = "pending-follower-request-count")]
pub async fn pending_follower_request_notification() -> Result<i64, ServerFnError> {
    let Ok(pool) = get_pool() else {
        return Ok(0);
    };
    let Ok(user) = get_request_user() else {
        return Ok(0);
    };
    let Ok(query) = Follower::pending_request_count(&pool, user.id).await else {
        return Ok(0);
    };
    Ok(query)
}

#[component]
pub fn FollowerCountComponent(follower_count: i64, following_count: i64) -> impl IntoView {
    view! {
        <section class="flex flex-wrap items-center ml-auto">
            <div class="">
                <IconUsers/>
            </div>
            <A class="py-1 px-2 whitespace-nowrap hover:bg-amber-200" href="followers">
                <span class="font-bold">{follower_count}</span>
                " followers"
            </A>
            <A class="py-1 px-2 whitespace-nowrap hover:bg-amber-200" href="following">
                <span class="font-bold">{following_count}</span>
                " following"
            </A>
        </section>
    }
}

#[component]
pub fn FollowerListItem(username: String) -> impl IntoView {
    let initial = username.chars().next().unwrap();
    view! {
        <a
            class="flex justify-between items-start p-2 mb-2 rounded bg-gray-100 hover:bg-gray-200"
            href=format!("/users/{}", username)
        >
            <div class="flex gap-4">
                <div class="flex justify-center items-center w-8 h-8 text-base font-bold text-white capitalize bg-red-500 rounded-full select-none hover:bg-red-700 shrink-0">
                    {initial}
                </div>
                <div class="capitalize">{username}</div>
            </div>
        </a>
    }
}

#[component]
pub fn UserFollowerListItem(
    username: String,
    created_at: String,
    children: Children,
) -> impl IntoView {
    let initial = username.chars().next().unwrap();
    view! {
        <div class="flex justify-between items-start p-2 mb-2 rounded bg-gray-100 hover:bg-gray-200">
            <section class="flex flex-1 gap-4">
                <a
                    href=format!("/users/{}", username)
                    class="flex justify-center items-center w-8 h-8 text-base font-bold text-white capitalize bg-red-500 rounded-full select-none hover:bg-red-700 shrink-0"
                >
                    {initial}
                </a>
                <div>
                    <div class="capitalize">{username}</div>
                    <div class="text-xs text-gray-600">{created_at}</div>
                </div>
            </section>
            <section class="flex gap-2">{children()}</section>
        </div>
    }
}
