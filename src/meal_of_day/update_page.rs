use leptos::*;
use leptos_router::*;

use crate::component::button::SubmitButton;
use crate::component::input::{NumberInput, TextInput};
use crate::component::template::{DetailPageTemplate, ErrorComponent, LoadingComponent};
use crate::util::param::get_slug;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

use super::detail_page::get_meal_of_day_detail;

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user, error::Error, meal_of_day::model::MealOfDay, setup::get_pool,
};

#[server(endpoint = "meal-of-day-update")]
pub async fn meal_of_day_update(
    slug: String,
    name: String,
    ordering: i32,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let object = MealOfDay::get_by_slug(&pool, &slug)
        .await?
        .ok_or(Error::NotFound)?;
    object.can_update(&user).await?;

    MealOfDay::validate(&name, ordering)?;

    let updated = MealOfDay::update(&pool, object.id, &name, ordering, user.id).await?;

    leptos_axum::redirect(&format!("/meal-of-day/{}", updated.slug));
    Ok(())
}

#[component]
pub fn MealOfDayUpdatePage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || get_slug(&params);

    let action = Action::<MealOfDayUpdate, _>::server();
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || extract_other_errors(action_value, &["name", "ordering"]);
    let non_field_errors = move || get_non_field_errors(action_value);

    let resource = Resource::new(slug, get_meal_of_day_detail);

    let response = move || {
        resource.and_then(|data| {
            let slug = data.slug.clone();
            let order = data.ordering.to_string();
            let name = data.name.clone();
            view! {
                <ActionForm action>
                    <input type="hidden" name="slug" value=slug/>
                    <TextInput name="name" value=name action_value/>
                    <NumberInput name="ordering" step="1" value=order action_value/>
                    <SubmitButton loading=action_loading label="Update Meal of Day"/>
                </ActionForm>
            }
        })
    };
    view! {
        <DetailPageTemplate title="Edit Meal of Day">
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
