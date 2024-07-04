use leptos::*;
use leptos_router::*;

use chrono::prelude::*;

use super::detail_table::ProfileDetailTable;
use super::model::{Profile, ProfileMetric};
use crate::component::template::{
    DetailPageTemplate, ErrorComponent, LoadingComponent, UpdateDeleteButtonRow,
};
use crate::util::param::{get_date, get_username};

#[cfg(feature = "ssr")]
use crate::{auth::model::User, auth::service::get_request_user, error::Error, setup::get_pool};

#[server(endpoint = "profile-detail-latest")]
pub async fn get_profile_detail_latest(
    username: String,
    date: NaiveDate,
) -> Result<Option<ProfileMetric>, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    User::check_view_permission(&pool, &user, &username).await?;

    let object = Profile::get_latest_by_username(&pool, &username, date)
        .await?
        .map(|profile| ProfileMetric::from(profile));

    Ok(object)
}

#[server(endpoint = "profile-detail")]
pub async fn get_profile_detail(
    username: String,
    date: NaiveDate,
) -> Result<ProfileMetric, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    User::check_view_permission(&pool, &user, &username).await?;

    let object = Profile::get_latest_by_username(&pool, &username, date)
        .await?
        .map(|profile| ProfileMetric::from(profile))
        .ok_or(Error::NotFound)?;

    Ok(object)
}

#[component]
pub fn ProfileDetailPage() -> impl IntoView {
    let params = use_params_map();
    let username = move || get_username(&params);
    let date = move || get_date(&params);
    let resource = Resource::new(
        move || (username(), date()),
        |(username, date)| get_profile_detail(username, date),
    );

    let response = move || {
        resource.and_then(|data| {
            view! {
                <ProfileDetailTable data=data.clone()/>
                <UpdateDeleteButtonRow/>
            }
        })
    };

    view! {
        <DetailPageTemplate title="Profile">
            <Transition fallback=LoadingComponent>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors/> }
                }>{response}</ErrorBoundary>
            </Transition>
        </DetailPageTemplate>
    }
}
