use leptos::*;
use leptos_router::*;

use super::detail_page::get_profile_detail;
use crate::component::button::SubmitButton;
use crate::component::template::{DetailPageTemplate, ErrorComponent, LoadingComponent};
use crate::util::param::{get_date, get_username};
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user, error::Error, profile::model::ProfileBase, setup::get_pool,
};

#[server(endpoint = "profile-delete")]
pub async fn profile_delete(username: String) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let object = ProfileBase::get_by_username(&pool, &username)
        .await?
        .ok_or(Error::NotFound)?;
    object.can_delete(&user).await?;

    ProfileBase::delete(&pool, object.id).await?;

    leptos_axum::redirect(&format!("/users/{username}"));
    Ok(())
}
#[component]
pub fn ProfileDeletePage() -> impl IntoView {
    let params = use_params_map();
    let username = move || get_username(&params);
    let date = move || get_date(&params);
    let resource = Resource::new(
        move || (username(), date()),
        |(username, date)| get_profile_detail(username, date),
    );

    let action = Action::<ProfileDelete, _>::server();
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || extract_other_errors(action_value, &["name"]);
    let non_field_errors = move || get_non_field_errors(action_value);

    let response = move || {
        resource.and_then(|data| {
            let id = data.id.to_string();
            let username = data.username.clone();
            view! {
                <p class="mb-4">"Are you sure you wish to delete this profile?"</p>
                <p class="mb-4">"Ths action cannot be undone."</p>
                <ActionForm action>
                    <input type="hidden" name="id" value=id/>
                    <input type="hidden" name="username" value=username/>
                    <SubmitButton loading=action_loading label="Delete"/>
                </ActionForm>
            }
        })
    };

    view! {
        <DetailPageTemplate title="Delete Profile">
            <div class="mb-4 text-red-500 font-bold">{action_error}</div>
            <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>
            <Transition fallback=LoadingComponent>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors/> }
                }>{response}</ErrorBoundary>
            </Transition>
        </DetailPageTemplate>
    }
}
