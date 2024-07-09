use leptos::*;
use leptos_router::*;

use uuid::Uuid;

use super::detail_page::get_muscle_group;
use crate::component::button::SubmitButton;
use crate::component::template::{DetailPageTemplate, ErrorComponent, LoadingComponent};
use crate::util::param::get_slug;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, error::Error, muscle_group::model::MuscleGroup};

#[server(endpoint = "muscle-group-delete")]
async fn muscle_group_delete(id: Uuid) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = expect_context::<sqlx::PgPool>();

    let object = MuscleGroup::get_by_id(&pool, id)
        .await?
        .ok_or(Error::NotFound)?;

    object.can_delete(&user).await?;

    MuscleGroup::delete(&pool, id).await?;

    leptos_axum::redirect("/exercises/muscle-groups");
    Ok(())
}

#[component]
pub fn MuscleGroupDeletePage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || get_slug(&params);

    let action = Action::<MuscleGroupDelete, _>::server();
    let action_loading = action.pending();
    let action_value = action.value();

    let action_error = move || extract_other_errors(action_value, &["name"]);
    let non_field_errors = move || get_non_field_errors(action_value);

    let resource = Resource::new(slug, get_muscle_group);

    let response = move || {
        resource.and_then(|data| {
            let id = data.id.to_string();
            view! {
                <p class="mb-4">"Are you sure you wish to delete this muscle group?"</p>
                <p class="mb-4">"Ths action cannot be undone."</p>
                <ActionForm action>
                    <input type="hidden" name="id" value=id/>
                    <SubmitButton loading=action_loading label="Delete Muscle Group"/>
                </ActionForm>
            }
        })
    };

    view! {
        <DetailPageTemplate title="Delete Muscle Group">
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
