use leptos::*;
use leptos_router::*;

use uuid::Uuid;

use crate::component::button::SubmitButton;
use crate::component::input::TextInput;
use crate::component::template::DetailPageTemplate;
use crate::muscle_group::select::{get_muscle_group_form_select, MuscleGroupFormSelect};
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, movement::model::Movement, setup::get_pool};

#[server(endpoint = "movement-create")]
pub async fn movement_create(muscle_group_id: Uuid, name: String) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    Movement::can_create(&user).await?;
    Movement::validate(&name)?;
    let object = Movement::create(&pool, muscle_group_id, &name, user.id).await?;

    leptos_axum::redirect(&format!("/exercises/{}", object.slug));
    Ok(())
}

#[component]
pub fn MovementCreatePage() -> impl IntoView {
    let action = Action::<MovementCreate, _>::server();
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || extract_other_errors(action_value, &["name", "muscle_group_id"]);
    let non_field_errors = move || get_non_field_errors(action_value);

    let resource = Resource::once(get_muscle_group_form_select);
    provide_context(resource);

    view! {
        <DetailPageTemplate title="New Exercise">
            <div class="mb-4 text-red-500 font-bold">{action_error}</div>
            <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>
            <ActionForm action>
                <TextInput action_value name="name" placeholder="Enter exercise name"/>
                <MuscleGroupFormSelect/>
                <SubmitButton loading=action_loading label="Create Exercise"/>
            </ActionForm>
        </DetailPageTemplate>
    }
}
