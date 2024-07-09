use leptos::*;
use leptos_meta::*;

use crate::follower::form::{FollowerAccept, FollowerRemove, FollowerUnfollow};
use crate::follower::received_request_list::ReceivedFollowerRequestList;
use crate::follower::sent_request_list::SentFollowerRequestList;
use crate::follower::user_follower_list::CurrentUserFollowerList;
use crate::follower::user_following_list::CurrentUserFollowingList;

#[component]
pub fn FollowerRequestListPage() -> impl IntoView {
    let action_accept: Action<FollowerAccept, Result<(), ServerFnError>> =
        Action::<FollowerAccept, _>::server();
    let action_remove: Action<FollowerRemove, Result<(), ServerFnError>> =
        Action::<FollowerRemove, _>::server();
    let action_unfollow: Action<FollowerUnfollow, Result<(), ServerFnError>> =
        Action::<FollowerUnfollow, _>::server();

    provide_context(action_accept);
    provide_context(action_remove);
    provide_context(action_unfollow);

    view! {
        <Title text="Followers"/>
        <div class="grid grid-cols-4 gap-4 lg:grid-cols-8">
            <div class="col-span-4 p-4 bg-white border shadow-sm">
                <ReceivedFollowerRequestList/>
            </div>

            <div class="col-span-4 p-4 bg-white border shadow-sm">
                <SentFollowerRequestList/>
            </div>

            <div class="col-span-4 p-4 bg-white border shadow-sm">
                <CurrentUserFollowerList/>
            </div>

            <div class="col-span-4 p-4 bg-white border shadow-sm">
                <CurrentUserFollowingList/>
            </div>
        </div>
    }
}
