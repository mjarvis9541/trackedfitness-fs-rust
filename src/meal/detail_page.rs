use std::collections::HashSet;

use leptos::server_fn::codec::GetUrl;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use uuid::Uuid;

use super::add_food_page::MealAddFoodComponent;
use super::model::Meal;
use crate::component::bulk_delete::BulkDeleteForm;
use crate::component::checkbox::CheckboxListItem;
use crate::component::template::{
    AutoListHeader, ErrorComponent, FoodListItemMacroHeader, ListLoadingComponent,
    ListNotFoundComponent, LoadingComponent, UpdateDeleteButtonRow,
};
use crate::food::nutrition_row::NutritionRow;
use crate::meal::add_food_page::MealAddFood;
use crate::meal_food::model::MealFood;
use crate::util::param::UuidParam;

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, error::Error, setup::get_pool};

#[server(endpoint = "meal-food-list", input = GetUrl)]
pub async fn get_meal_food_list(meal_id: Uuid) -> Result<Vec<MealFood>, ServerFnError> {
    get_request_user()?;
    let pool = get_pool()?;
    let query = MealFood::all_by_meal_id(&pool, meal_id).await?;
    Ok(query)
}

#[server(endpoint = "meal-detail", input = GetUrl)]
pub async fn get_meal_detail(id: Uuid) -> Result<Meal, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let object = Meal::get_by_id(&pool, id).await?.ok_or(Error::NotFound)?;

    object.can_view(&user)?;
    Ok(object)
}

#[component]
pub fn MealDetailPage() -> impl IntoView {
    let action_bulk_delete = Action::server();
    let action_add_food = Action::<MealAddFood, _>::server();
    provide_context(action_add_food);

    let params = use_params::<UuidParam>();
    let id = move || params.with(|p| p.as_ref().map(|p| p.id).unwrap_or_default());

    let resource = Resource::new(
        move || {
            (
                id(),
                action_add_food.version().get(),
                action_bulk_delete.version().get(),
            )
        },
        |(id, _, _)| get_meal_detail(id),
    );
    let meal_food_resource = Resource::new(
        move || {
            (
                id(),
                action_add_food.version().get(),
                action_bulk_delete.version().get(),
            )
        },
        |(id, _, _)| get_meal_food_list(id),
    );

    let all_items = RwSignal::new(HashSet::<String>::new());
    let checked_items = RwSignal::new(HashSet::<String>::new());

    let response = move || {
        resource.and_then(|data| {
            view! {
                <header class="flex flex-wrap gap-2 justify-between items-start mb-4">
                    <div>
                        <h1 class="text-xl font-bold">{&data.name}</h1>
                        <p class="text-gray-400">"Food: " {data.food_count}</p>
                    </div>
                    <UpdateDeleteButtonRow/>
                </header>
            }
        })
    };
    let meal_food_response = move || {
        meal_food_resource.and_then(|results| {
            if results.is_empty() {
                view! { <ListNotFoundComponent/> }
            } else {
                let ids: HashSet<String> = results.iter().map(|item| item.id.to_string()).collect();
                all_items.update(|set| set.extend(ids));
                results
                    .iter()
                    .map(|inner| {
                        view! { <MealFoodListItem data=inner checked_items/> }
                    })
                    .collect_view()
            }
        })
    };
    let meal_total_response = move || {
        resource.and_then(|data| {
            let nutrition = &data.nutrition;
            view! {
                <div class="flex col-span-4 items-center py-1 px-2 pb-0 font-bold bg-gray-100 lg:col-span-5">
                    "Total"
                </div>
                <NutritionRow data=nutrition/>
            }
        })
    };
    view! {
        <Title text="Meal Detail"/>
        <main class="md:p-4">

            <section class="p-4 mb-4 bg-white border">
                <Transition fallback=LoadingComponent>
                    <ErrorBoundary fallback=|errors| {
                        view! { <ErrorComponent errors/> }
                    }>{response}</ErrorBoundary>
                </Transition>

                <section class="grid grid-cols-4 mb-4 lg:grid-cols-checkbox-12">
                    <AutoListHeader all_items checked_items>
                        "Food"
                        ""
                        ""
                        "Quantity"
                        "Calories"
                        "Protein"
                        "Carbs"
                        "Fat"
                        "Sat.Fat"
                        "Sugars"
                        "Fibre"
                        "Salt"
                    </AutoListHeader>
                    <Transition fallback=ListLoadingComponent>{meal_food_response}</Transition>
                    <Transition>{meal_total_response}</Transition>
                </section>

                <BulkDeleteForm table="meal_food" action=action_bulk_delete checked_items/>
            </section>

            <section class="p-4 bg-white border">
                <MealAddFoodComponent/>
            </section>

        </main>
    }
}

#[component]
fn MealFoodListItem<'a>(
    data: &'a MealFood,
    checked_items: RwSignal<HashSet<String>>,
) -> impl IntoView {
    let title = format!("{}, {}", data.food_name, data.brand_name);
    let serving = format!("{:.0}{}", data.data_value, data.data_measurement);
    let nutrition = &data.nutrition;
    view! {
        <div class="contents group">
            <div class="hidden justify-center items-center py-2 px-2 lg:flex group-hover:bg-gray-200 group-odd:bg-gray-50">
                <CheckboxListItem id=data.id.to_string() checked_items/>
            </div>
            <div class="flex col-span-3 items-center py-1 px-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
                <A class="hover:underline" href=data.id.to_string()>
                    {title}
                </A>
            </div>
            <div class="flex justify-end items-end py-2 px-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
                <A class="hover:underline" href=format!("{}/update", data.id)>
                    {serving}
                </A>
            </div>
            <FoodListItemMacroHeader/>
            <NutritionRow data=nutrition/>
        </div>
    }
}
