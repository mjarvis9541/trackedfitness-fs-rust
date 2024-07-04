use leptos::*;
use leptos_router::*;

use super::detail_page::get_movement_detail;
use crate::component::button::SubmitButton;
use crate::component::input::TextInputImproved;
use crate::component::template::{DetailPageTemplate, ErrorComponent, LoadingComponent};
use crate::muscle_group::select::MuscleGroupFormSelect;
use crate::util::param::get_slug;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};
use uuid::Uuid;

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user, error::Error, movement::model::MovementBase, setup::get_pool,
};

#[server(endpoint = "movement-update")]
pub async fn movement_update(
    id: Uuid,
    muscle_group_id: Uuid,
    name: String,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    let object = MovementBase::get_by_id(&pool, id)
        .await?
        .ok_or(Error::NotFound)?;
    object.can_update(&user).await?;
    MovementBase::validate(&name)?;
    MovementBase::update(&pool, object.id, &name, muscle_group_id, user.id).await?;

    leptos_axum::redirect(&format!("/exercises/{}", object.slug));
    Ok(())
}

#[component]
pub fn MovementUpdatePage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || get_slug(&params);

    let action = Action::<MovementUpdate, _>::server();
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || extract_other_errors(action_value, &["name"]);
    let non_field_errors = move || get_non_field_errors(action_value);

    let resource = Resource::new(slug, get_movement_detail);

    let response = move || {
        resource.and_then(|data| {
            let muscle_group_id = data.muscle_group_id;
            let id = data.id.to_string();
            let name = data.name.clone();
            view! {
                <ActionForm action>
                    <input type="hidden" name="id" value=id/>
                    <TextInputImproved
                        name="name"
                        value=name
                        action_value
                        placeholder="Enter exercise name"
                    />
                    <MuscleGroupFormSelect selected=muscle_group_id/>
                    <SubmitButton loading=action_loading/>
                </ActionForm>
            }
        })
    };

    view! {
        <DetailPageTemplate title="Edit Exercise">
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
