use leptos::*;
use leptos_router::*;

use uuid::Uuid;

use super::model::MealFood;
use crate::component::template::{
    DetailPageTemplate, ErrorComponent, LoadingComponent, UpdateDeleteButtonRow,
};
use crate::diet::detail_page::NutritionTable;
use crate::food::router::MealFoodParam;

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user, error::Error, meal::model::MealBase, setup::get_pool,
};

#[server(endpoint = "meal-food-detail")]
pub async fn get_meal_food_detail(meal_food_id: Uuid) -> Result<MealFood, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let meal_food = MealFood::get_object_or_404(&pool, meal_food_id).await?;
    let meal = MealBase::get_by_id(&pool, meal_food.meal_id)
        .await?
        .ok_or(Error::NotFound)?;

    meal.can_view(&user)?;

    Ok(meal_food)
}

#[component]
pub fn MealFoodDetailPage() -> impl IntoView {
    let params = use_params::<MealFoodParam>();
    let id = move || params.with(|p| p.as_ref().map(|p| p.meal_food_id).unwrap_or_default());

    let resource = Resource::new(id, get_meal_food_detail);
    let response = move || resource.and_then(|data| view! { <MealFoodDetail data=data.clone()/> });

    view! {
        <DetailPageTemplate title="Meal Food">
            <Transition fallback=LoadingComponent>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors/> }
                }>{response}</ErrorBoundary>
            </Transition>
        </DetailPageTemplate>
    }
}

#[component]
pub fn MealFoodDetail(data: MealFood) -> impl IntoView {
    let per = format!("Per {:.0}{}", data.data_value, data.data_measurement);
    let nutrition = &data.nutrition;

    view! {
        <h1 class="mb-4 text-xl font-bold">{data.food_name}</h1>
        <NutritionTable data=nutrition per/>
        <UpdateDeleteButtonRow/>
    }
}
