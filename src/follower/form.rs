use leptos::*;
use leptos_router::*;

use crate::component::button::Button;
use crate::component::icon::{IconUserMinus, IconUserPlus};
use crate::error_extract::{extract_error_message, process_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, follower::model::Follower, setup::get_pool};

#[server(endpoint = "follower-request")]
pub async fn follower_request(username: String) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    if user.username == username {
        return Err(ServerFnError::new("You cannot follow yourself"));
    }
    Follower::request(&pool, &username, &user.username).await?;
    Ok(())
}

#[server(endpoint = "follower-accept")]
pub async fn follower_accept(username: String) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    if user.username == username {
        return Err(ServerFnError::new("You cannot follow yourself"));
    }
    Follower::accept(&pool, &username, &user.username).await?;
    Ok(())
}

#[server(endpoint = "follower-remove")]
pub async fn follower_remove(username: String) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    if user.username == username {
        return Err(ServerFnError::new("You cannot unfollow yourself"));
    }
    Follower::remove(&pool, &username, &user.username).await?;
    Ok(())
}

#[server(endpoint = "follower-unfollow")]
pub async fn follower_unfollow(username: String) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    if user.username == username {
        return Err(ServerFnError::new("You cannot unfollow yourself"));
    }
    Follower::unfollow(&pool, &username, &user.username).await?;
    Ok(())
}

#[component]
pub fn FollowerRequestForm(
    username: String,
    action: Action<FollowerRequest, Result<(), ServerFnError>>,
) -> impl IntoView {
    let error = move || extract_error_message(&action);
    let non_field_errors = move || process_non_field_errors(error);
    view! {
        <div>
            {error} {non_field_errors} <ActionForm action class="contents">
                <input type="hidden" name="username" value=username/>
                <Button label="Follow">
                    <IconUserPlus/>
                </Button>
            </ActionForm>
        </div>
    }
}

#[component]
pub fn FollowerAcceptForm(
    username: String,
    action: Action<FollowerAccept, Result<(), ServerFnError>>,
) -> impl IntoView {
    let error = move || extract_error_message(&action);
    let non_field_errors = move || process_non_field_errors(error);
    view! {
        <div>
            {error} {non_field_errors} <ActionForm action class="contents">
                <input type="hidden" name="username" value=username/>
                <Button label="Accept">
                    <IconUserPlus/>
                </Button>
            </ActionForm>
        </div>
    }
}

#[component]
pub fn FollowerRemoveForm(
    label: &'static str,
    username: String,
    action: Action<FollowerRemove, Result<(), ServerFnError>>,
) -> impl IntoView {
    let error = move || extract_error_message(&action);
    let non_field_errors = move || process_non_field_errors(error);
    view! {
        <div>
            {error} {non_field_errors} <ActionForm action class="contents">
                <input type="hidden" name="username" value=username/>
                <Button label>
                    <IconUserMinus/>
                </Button>
            </ActionForm>
        </div>
    }
}

#[component]
pub fn FollowerUnfollowForm(
    label: &'static str,
    username: String,
    action: Action<FollowerUnfollow, Result<(), ServerFnError>>,
) -> impl IntoView {
    let error = move || extract_error_message(&action);
    let non_field_errors = move || process_non_field_errors(error);
    view! {
        <div>
            {error} {non_field_errors} <ActionForm action class="contents">
                <input type="hidden" name="username" value=username/>
                <Button label>
                    <IconUserMinus/>
                </Button>
            </ActionForm>
        </div>
    }
}
