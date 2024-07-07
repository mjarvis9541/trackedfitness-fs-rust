use leptos::*;
use leptos_router::*;

use uuid::Uuid;

use super::detail_page::get_muscle_group;
use crate::component::button::SubmitButton;
use crate::component::input::TextInput;
use crate::component::template::{DetailPageTemplate, ErrorComponent, LoadingComponent};
use crate::util::param::get_slug;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user, error::Error, muscle_group::model::MuscleGroupBase,
    setup::get_pool,
};

#[server(endpoint = "muscle-group-update")]
async fn muscle_group_update(id: Uuid, name: String) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    let object = MuscleGroupBase::get_by_id(&pool, id)
        .await?
        .ok_or(Error::NotFound)?;
    object.can_update(&user).await?;
    MuscleGroupBase::validate(&name)?;
    let updated = MuscleGroupBase::update(&pool, object.id, &name, user.id).await?;
    leptos_axum::redirect(&format!("/exercises/muscle-groups/{}", updated.slug));
    Ok(())
}

#[component]
pub fn MuscleGroupUpdatePage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || get_slug(&params);

    let action = Action::<MuscleGroupUpdate, _>::server();
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || extract_other_errors(action_value, &["name"]);
    let non_field_errors = move || get_non_field_errors(action_value);

    let resource = Resource::new(slug, get_muscle_group);
    let response = move || {
        resource.and_then(|data| {
            let id = data.id.to_string();
            let name = data.name.clone();
            view! {
                <ActionForm action>
                    <input type="hidden" name="id" value=id/>
                    <TextInput
                        name="name"
                        value=name
                        action_value
                        placeholder="Enter muscle group name"
                    />
                    <SubmitButton loading=action_loading/>
                </ActionForm>
            }
        })
    };

    view! {
        <DetailPageTemplate title="Edit Muscle Group">
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
