use leptos::*;
use leptos_router::*;

use super::detail_page::get_meal_of_day_detail;
use crate::component::button::SubmitButton;
use crate::component::template::{DetailPageTemplate, ErrorComponent, LoadingComponent};
use crate::util::param::get_slug;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user, error::Error, meal_of_day::model::MealOfDay, setup::get_pool,
};

#[server(endpoint = "meal-of-day-delete")]
pub async fn meal_of_day_delete(slug: String) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let object = MealOfDay::get_by_slug(&pool, &slug)
        .await?
        .ok_or(Error::NotFound)?;
    object.can_delete(&user).await?;

    MealOfDay::delete(&pool, object.id).await?;
    leptos_axum::redirect("/meal-of-day");
    Ok(())
}

#[component]
pub fn MealOfDayDeletePage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || get_slug(&params);

    let action = Action::<MealOfDayDelete, _>::server();
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || extract_other_errors(action_value, &["name", "ordering"]);
    let non_field_errors = move || get_non_field_errors(action_value);

    let resource = Resource::new(slug, get_meal_of_day_detail);
    let response = move || {
        resource.and_then(|data| {
            let slug = data.slug.clone();
            view! {
                <p class="mb-4">"Are you sure you wish to delete this meal of day?"</p>
                <p class="mb-4">"Ths action cannot be undone."</p>
                <ActionForm action>
                    <input type="hidden" name="slug" value=slug/>
                    <SubmitButton loading=action_loading label="Delete Meal of Day"/>
                </ActionForm>
            }
        })
    };

    view! {
        <DetailPageTemplate title="Delete Meal of Day">
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
