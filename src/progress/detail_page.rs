use leptos::*;
use leptos_router::*;

use chrono::prelude::*;

use crate::component::template::{
    DetailPageTemplate, ErrorComponent, LoadingComponent, UpdateDeleteButtonRow,
};
use crate::util::param::{get_date, get_username};

use super::detail_table::ProgressDetailTable;
use super::model::ProgressQuery;

#[cfg(feature = "ssr")]
use crate::{auth::model::User, auth::service::get_request_user, error::Error, setup::get_pool};

#[server(endpoint = "progress-detail-latest")]
pub async fn get_progress_detail_latest(
    username: String,
    date: NaiveDate,
) -> Result<Option<ProgressQuery>, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    User::check_view_permission(&pool, &user, &username).await?;
    let query = ProgressQuery::get_latest_by_username_date(&pool, &username, date).await?;
    Ok(query)
}

#[server(endpoint = "progress-detail")]
pub async fn get_progress_detail(
    username: String,
    date: NaiveDate,
) -> Result<ProgressQuery, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    User::check_view_permission(&pool, &user, &username).await?;
    let query = ProgressQuery::get_by_username_date(&pool, &username, date)
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
