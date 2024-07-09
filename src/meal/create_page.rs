use leptos::*;
use leptos_router::*;

use crate::component::button::SubmitButton;
use crate::component::input::TextInput;
use crate::component::template::DetailPageTemplate;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, meal::model::Meal, setup::get_pool};

#[server(endpoint = "meal-create")]
pub async fn meal_create(name: String) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    Meal::can_create(&user).await?;
    Meal::validate(&name)?;

    let object = Meal::create(&pool, user.id, &name, user.id).await?;

    leptos_axum::redirect(&format!("/food/meals/{}", object.id));
    Ok(())
}

#[component]
pub fn MealCreatePage() -> impl IntoView {
    let action = Action::<MealCreate, _>::server();
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || extract_other_errors(action_value, &["name"]);
    let non_field_errors = move || get_non_field_errors(action_value);
    view! {
        <DetailPageTemplate title="New Meal">
            <div class="mb-4 text-red-500 font-bold">{action_error}</div>
            <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>
            <ActionForm action>
                <TextInput action_value name="name" placeholder="Enter meal name"/>
                <SubmitButton loading=action_loading label="Create Meal"/>
            </ActionForm>
        </DetailPageTemplate>
    }
}
