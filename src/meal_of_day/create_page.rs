use leptos::*;
use leptos_router::*;

use crate::component::button::SubmitButton;
use crate::component::input::{NumberInput, TextInputImproved};
use crate::component::template::DetailPageTemplate;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, meal_of_day::model::MealOfDay, setup::get_pool};

#[server(endpoint = "meal-of-day-create")]
pub async fn meal_of_day_create(name: String, ordering: i32) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    MealOfDay::can_create(&user).await?;
    MealOfDay::validate(&name, ordering)?;

    let object = MealOfDay::create(&pool, &name, ordering, user.id).await?;

    leptos_axum::redirect(&format!("/meal-of-day/{}", object.slug));
    Ok(())
}

#[component]
pub fn MealOfDayCreatePage() -> impl IntoView {
    let action = Action::<MealOfDayCreate, _>::server();

    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || extract_other_errors(action_value, &["name", "ordering"]);
    let non_field_errors = move || get_non_field_errors(action_value);

    view! {
        <DetailPageTemplate title="New Meal of Day">
            <div class="mb-4 text-red-500 font-bold">{action_error}</div>
            <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>
            <ActionForm action>
                <TextInputImproved action_value name="name" placeholder="Enter meal of day name"/>
                <NumberInput
                    action_value
                    name="ordering"
                    step="1"
                    placeholder="Enter order of meal, 1-100"
                />
                <SubmitButton loading=action_loading label="Create Meal of Day"/>
            </ActionForm>
        </DetailPageTemplate>
    }
}
