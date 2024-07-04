use leptos::*;
use leptos_router::*;

use crate::component::template::{ErrorComponent, LoadingComponent};
use crate::util::param::get_username;

use super::model::ProfileImage;

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, error::Error, setup::get_pool};

#[server]
pub async fn get_profile_image(username: String) -> Result<ProfileImage, ServerFnError> {
    let _user = get_request_user()?;
    let pool = get_pool()?;
    let object = ProfileImage::get_by_username(&pool, &username)
        .await?
        .ok_or(Error::NotFound)?;
    Ok(object)
}

#[component]
pub fn ProfileImageComponent() -> impl IntoView {
    let params = use_params_map();
    let username = move || get_username(&params);
    let resource = Resource::new(username, get_profile_image);
    let response = move || {
        resource.and_then(|data| {
            let image_url = data.image_location.clone().map_or_else(
                || "/images/profile/default.jpg".to_string(),
                |image| format!("/images/profile/{}", image),
            );
            view! {
                <div class="w-16 h-16 rounded-full overflow-hidden">
                    <img src=image_url alt="Profile Image" class="w-full h-full object-cover"/>
                </div>
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
