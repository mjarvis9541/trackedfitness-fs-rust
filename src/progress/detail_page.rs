use leptos::*;
use leptos_router::*;

use chrono::prelude::*;

use crate::component::template::{
    DetailPageTemplate, ErrorComponent, LoadingComponent, UpdateDeleteButtonRow,
};
use crate::util::param::{get_date, get_username};

use super::detail_table::ProgressDetailTable;
use super::model::Progress;

#[cfg(feature = "ssr")]
use crate::{auth::model::User, auth::service::get_request_user, error::Error, setup::get_pool};

#[server]
pub async fn get_progress_detail_latest(
    username: String,
    date: NaiveDate,
) -> Result<Option<Progress>, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    User::check_view_permission(&pool, &user, &username).await?;
    let query = Progress::get_latest_by_username_date(&pool, &username, date).await?;
    Ok(query)
}

#[server]
pub async fn get_progress_detail(
    username: String,
    date: NaiveDate,
) -> Result<Progress, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    User::check_view_permission(&pool, &user, &username).await?;
    let query = Progress::get_by_username_date(&pool, &username, date)
        .await?
        .ok_or(Error::NotFound)?;
    Ok(query)
}

#[component]
pub fn ProgressDetailPage() -> impl IntoView {
    let params = use_params_map();
    let username = move || get_username(&params);
    let date = move || get_date(&params);
    let resource = Resource::new(
        move || (username(), date()),
        |(username, date)| get_progress_detail(username, date),
    );
    let response = move || {
        resource.and_then(|data| {
            view! {
                <ProgressDetailTable data=data.clone()/>
                <UpdateDeleteButtonRow/>
            }
        })
    };
    view! {
        <DetailPageTemplate title="Progress">
            <Transition fallback=LoadingComponent>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors=errors/> }
                }>{response}</ErrorBoundary>
            </Transition>
        </DetailPageTemplate>
    }
}
