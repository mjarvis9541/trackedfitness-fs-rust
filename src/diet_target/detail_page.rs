use leptos::*;
use leptos_router::*;

use chrono::prelude::*;

use super::detail_table::DietTargetDetailTable;
use super::model::DietTarget;
use crate::component::template::{
    DetailPageTemplate, ErrorComponent, LoadingComponent, UpdateDeleteButtonRow,
};
use crate::util::param::{get_date, get_username};

#[cfg(feature = "ssr")]
use crate::{auth::model::User, auth::service::get_request_user, error::Error, setup::get_pool};

#[server(endpoint = "diet-target-detail-latest")]
pub async fn get_diet_target_detail_latest(
    username: String,
    date: NaiveDate,
) -> Result<Option<DietTarget>, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    User::check_view_permission(&pool, &user, &username).await?;
    let object = DietTarget::get_latest_by_username_date(&pool, &username, date).await?;
    Ok(object)
}

#[server(endpoint = "diet-target-detail")]
pub async fn get_diet_target_detail(
    username: String,
    date: NaiveDate,
) -> Result<DietTarget, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    User::check_view_permission(&pool, &user, &username).await?;
    let object = DietTarget::get_by_username_date(&pool, &username, date)
        .await?
        .ok_or(Error::NotFound)?;
    Ok(object)
}

#[component]
pub fn DietTargetDetailPage() -> impl IntoView {
    let params = use_params_map();
    let username = move || get_username(&params);
    let date = move || get_date(&params);
    let resource = Resource::new(
        move || (username(), date()),
        |(username, date)| get_diet_target_detail(username, date),
    );
    let response = move || {
        resource.and_then(|data| {
            view! {
                <DietTargetDetailTable data=data.clone()/>
                <UpdateDeleteButtonRow/>
            }
        })
    };
    view! {
        <DetailPageTemplate title="Diet Target">
            <Transition fallback=LoadingComponent>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors/> }
                }>{response}</ErrorBoundary>
            </Transition>
        </DetailPageTemplate>
    }
}
