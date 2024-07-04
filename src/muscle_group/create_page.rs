use leptos::*;
use leptos_router::*;

use crate::component::button::SubmitButton;
use crate::component::input::TextInputImproved;
use crate::component::template::DetailPageTemplate;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, muscle_group::model::MuscleGroupBase};

#[server(endpoint = "muscle-group-create")]
async fn muscle_group_create(name: String) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = expect_context::<sqlx::PgPool>();
    MuscleGroupBase::can_create(&user).await?;
    MuscleGroupBase::validate(&name)?;
    let object = MuscleGroupBase::create(&pool, &name, user.id).await?;
    leptos_axum::redirect(&format!("/exercises/muscle-groups/{}", object.slug));
    Ok(())
}

#[component]
pub fn MuscleGroupCreatePage() -> impl IntoView {
    let action = Action::<MuscleGroupCreate, _>::server();
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || extract_other_errors(action_value, &["name"]);
    let non_field_errors = move || get_non_field_errors(action_value);

    view! {
        <DetailPageTemplate title="New Muscle Group">
            <div class="mb-4 text-red-500 font-bold">{action_error}</div>
            <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>
            <ActionForm action>
                <TextInputImproved action_value name="name" placeholder="Enter muscle group name"/>
                <SubmitButton loading=action_loading label="Create Muscle Group"/>
            </ActionForm>
        </DetailPageTemplate>
    }
}
