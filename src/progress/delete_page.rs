use leptos::*;
use leptos_router::*;

use chrono::prelude::*;

use crate::component::button::SubmitButton;
use crate::component::template::{DetailPageTemplate, ErrorComponent, LoadingComponent};
use crate::util::param::{get_date, get_username};
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

use super::detail_page::get_progress_detail;

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user, error::Error, progress::model::Progress, setup::get_pool,
};

#[server(endpoint = "progress-delete")]
pub async fn progress_delete(username: String, date: NaiveDate) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let object = Progress::get_by_username_date(&pool, &username, date)
        .await?
        .ok_or(Error::NotFound)?;
    object.can_delete(&user).await?;

    Progress::delete(&pool, object.id).await?;

    leptos_axum::redirect(&format!("/users/{}/{}", username, date));

    Ok(())
}

#[component]
pub fn ProgressDeletePage() -> impl IntoView {
    let params = use_params_map();
    let username = move || get_username(&params);
    let date = move || get_date(&params);
    let resource = Resource::new(
        move || (username(), date()),
        |(username, date)| get_progress_detail(username, date),
    );

    let action = Action::<ProgressDelete, _>::server();
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error =
        move || extract_other_errors(action_value, &["non_field_errors", "username", "date"]);
    let non_field_errors = move || get_non_field_errors(action_value);

    let response = move || {
        resource.and_then(|data| {
            let username = data.username.clone();
            let date = data.date.to_string();
            view! {
                <p class="mb-4">"Are you sure you wish to delete this progress log entry?"</p>
                <p class="mb-4">"Ths action cannot be undone."</p>
                <ActionForm action>
                    <input type="hidden" name="username" value=username/>
                    <input type="hidden" name="date" value=date/>
                    <SubmitButton loading=action_loading label="Delete Progress"/>
                </ActionForm>
            }
        })
    };
    view! {
        <DetailPageTemplate title="Delete Progress">
            <div class="mb-4 text-red-500 font-bold">{action_error}</div>
            <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>
            <Transition fallback=LoadingComponent>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors=errors/> }
                }>{response}</ErrorBoundary>
            </Transition>
        </DetailPageTemplate>
    }
}
