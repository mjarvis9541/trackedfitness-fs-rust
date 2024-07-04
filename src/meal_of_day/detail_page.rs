use leptos::*;
use leptos_router::*;

use crate::component::template::{
    DetailPageTemplate, ErrorComponent, LoadingComponent, UpdateDeleteButtonRow,
};
use crate::util::datetime::format_datetime;
use crate::util::param::get_slug;

use super::model::MealOfDay;

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, error::Error, setup::get_pool};

#[server(endpoint = "meal-of-day-detail")]
pub async fn get_meal_of_day_detail(slug: String) -> Result<MealOfDay, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let object = MealOfDay::get_by_slug(&pool, &slug)
        .await?
        .ok_or(Error::NotFound)?;
    object.can_view(&user).await?;

    Ok(object)
}

#[component]
pub fn MealOfDayDetailPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || get_slug(&params);

    let resource = Resource::new(slug, get_meal_of_day_detail);
    let response = move || resource.and_then(|data| view! { <MealOfDayDetail data/> });

    view! {
        <DetailPageTemplate title="Meal of Day">
            <Transition fallback=LoadingComponent>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors/> }
                }>{response}</ErrorBoundary>
            </Transition>
        </DetailPageTemplate>
    }
}

#[component]
pub fn MealOfDayDetail<'a>(data: &'a MealOfDay) -> impl IntoView {
    let created_at = format_datetime(&Some(data.created_at));
    let updated_at = format_datetime(&data.updated_at);
    view! {
        <header>
            <h1 class="mb-4 text-xl font-bold capitalize">{&data.name}</h1>
        </header>

        <table class="mb-4 w-full border-collapse">
            <tbody>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Slug"</th>
                    <td class="p-2 w-1/2 text-right border">{&data.slug}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Order"</th>
                    <td class="p-2 w-1/2 text-right border">{data.ordering}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Created"</th>
                    <td class="p-2 w-1/2 text-right border">{created_at}</td>
                </tr>
                <tr>
                    <th class="p-2 w-1/2 text-left border">"Updated"</th>
                    <td class="p-2 w-1/2 text-right border">{updated_at}</td>
                </tr>
            </tbody>
        </table>

        <UpdateDeleteButtonRow/>
    }
}
