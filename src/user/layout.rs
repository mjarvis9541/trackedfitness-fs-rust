use leptos::*;
use leptos_router::*;

use crate::auth::context::CanEditContext;
use crate::component::link::{CircularIconLink, CircularIconLinkVariant};
use crate::component::template::{ErrorComponent, LoadingComponent};
use crate::follower::component::FollowerCountComponent;
use crate::follower::form::{
    FollowerRequest, FollowerRequestForm, FollowerUnfollow, FollowerUnfollowForm,
};
use crate::follower::status::FollowerStatus;
use crate::user::model::UserQuery;
use crate::util::param::UsernameParam;

use super::user_navigation::UserNavigation;

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, setup::get_pool};

#[server]
pub async fn get_user_layout(username: String) -> Result<UserQuery, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    let query = UserQuery::get(&pool, &username, user.id).await?;
    Ok(query)
}
pub type UserLayoutResource = Resource<(String, usize, usize), Result<UserQuery, ServerFnError>>;

#[component]
pub fn UserLayout() -> impl IntoView {
    let action_request = Action::<FollowerRequest, _>::server();
    let action_unfollow = Action::<FollowerUnfollow, _>::server();

    let can_edit = CanEditContext::new();
    provide_context(can_edit);

    let params = use_params::<UsernameParam>();
    let username =
        move || params.with(|p| p.as_ref().map(|p| p.username.clone()).unwrap_or_default());
    let resource: UserLayoutResource = Resource::new(
        move || {
            (
                username(),
                action_request.version().get(),
                action_unfollow.version().get(),
            )
        },
        |(username, _, _)| get_user_layout(username),
    );
    provide_context(resource);
    let response = move || {
        resource.and_then(|data| {
            can_edit.can_edit.update(|value| *value = data.is_self);

            let initial = data.username.chars().next().unwrap();
            let show_user_nav = data.is_self || data.follower_status == FollowerStatus::Accepted;
            let can_request = !data.is_self && data.follower_status.can_request();
            let can_unfollow = !data.is_self && data.follower_status.can_unfollow();
            let unfollow_wording = data.follower_status.get_unfollow_wording();
            view! {
                <nav class="flex flex-wrap gap-2 items-start py-2 px-4 bg-gray-200">
                    <section class="flex gap-4 items-center">
                        <CircularIconLink initial variant=CircularIconLinkVariant::Large href=""/>
                        <div>
                            <h1 class="text-xl font-bold capitalize">{&data.username}</h1>
                            <p class="text-gray-500">{&data.name}</p>
                        </div>
                    </section>
                    <FollowerCountComponent
                        follower_count=data.follower_count
                        following_count=data.following_count
                    />
                    <section class=("hidden", !show_user_nav)>
                        <UserNavigation/>
                    </section>
                    <section class="flex flex-wrap gap-2">
                        <div class=("hidden", !can_request)>
                            <FollowerRequestForm
                                username=data.username.clone()
                                action=action_request
                            />
                        </div>
                        <div class=("hidden", !can_unfollow)>
                            <FollowerUnfollowForm
                                username=data.username.clone()
                                action=action_unfollow
                                label=unfollow_wording
                            />
                        </div>
                    </section>
                </nav>
                <Outlet/>
            }
        })
    };

    view! {
        <Transition fallback=LoadingComponent>
            <ErrorBoundary fallback=|errors| {
                view! { <ErrorComponent errors/> }
            }>{response}</ErrorBoundary>
        </Transition>
    }
}
