use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use super::model::Food;
use super::to_diet_form::FoodToDietForm;
use super::to_meal_form::FoodToMealForm;
use crate::component::template::{ErrorComponent, LoadingComponent, UpdateDeleteButtonRow};
use crate::util::datetime::format_datetime;
use crate::util::param::get_slug;

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, error::Error, setup::get_pool};

#[server]
pub async fn get_food_detail(slug: String) -> Result<Food, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    let food = Food::get_by_slug(&pool, &slug)
        .await?
        .ok_or(Error::NotFound)?;
    food.can_view(&user)?;
    Ok(food)
}

#[component]
pub fn FoodDetailPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || get_slug(&params);

    let resource = Resource::new(slug, get_food_detail);
    let response = move || resource.and_then(|data| view! { <FoodDetailComponent data=data/> });
    let diet_form_response = move || {
        resource.and_then(|data| {
            let food_id = data.id.to_string();
            let quantity = format!("Quantity ({})", data.data_measurement);
            let data_value = data.data_value;
            view! { <FoodToDietForm food_id quantity data_value/> }
        })
    };
    let meal_form_response = move || {
        resource.and_then(|data| {
            let food_id = data.id.to_string();
            let quantity = format!("Quantity ({})", data.data_measurement);
            let data_value = data.data_value;
            view! { <FoodToMealForm food_id quantity data_value/> }
        })
    };
    view! {
        <Title text="Food Detail"/>
        <main class="p-4">
            <div class="grid grid-cols-4 gap-4 md:grid-cols-8 lg:grid-cols-12">
                <div class="col-span-4">
                    <div class="p-4 bg-white border">
                        <h2 class="mb-2 text-base font-bold">"Food Detail"</h2>
                        <Transition fallback=LoadingComponent>
                            <ErrorBoundary fallback=|errors| {
                                view! { <ErrorComponent errors/> }
                            }>{response}</ErrorBoundary>
                        </Transition>
                    </div>
                </div>
                <div class="col-span-4">
                    <div class="p-4 mb-4 bg-white border">
                        <h2 class="mb-2 text-base font-bold">"Add to Diet Log"</h2>
                        <Transition fallback=LoadingComponent>
                            <ErrorBoundary fallback=|errors| {
                                view! { <ErrorComponent errors/> }
                            }>{diet_form_response}</ErrorBoundary>
                        </Transition>
                    </div>
                </div>
                <div class="col-span-4">
                    <div class="p-4 bg-white border">
                        <h2 class="mb-2 text-base font-bold">"Add to Meal"</h2>
                        <Transition fallback=LoadingComponent>
                            <ErrorBoundary fallback=|errors| {
                                view! { <ErrorComponent errors/> }
                            }>{meal_form_response}</ErrorBoundary>
                        </Transition>
                    </div>
                </div>
            </div>
        </main>
    }
}

#[component]
pub fn FoodDetailComponent<'a>(data: &'a Food) -> impl IntoView {
    let created_at = format_datetime(&Some(data.created_at));
    let updated_at = format_datetime(&data.updated_at);

    let title = format!(
        "{}, {:.0}{}",
        data.name, data.data_value, data.data_measurement
    );
    let brand_title = data.brand_name.clone();

    view! {
        <header>
            <h1 class="text-base font-bold">{title}</h1>
            <p class="mb-4 text-gray-500">
                <A href=format!("/brands/{}", data.brand_slug)>{brand_title}</A>
            </p>
        </header>

        <h3 class="mb-2 text-base font-bold">"Nutrition Information"</h3>
        <div class="mb-4">
            <table class="w-full border border-collapse table-fixed">
                <thead>
                    <tr>
                        <th class="p-2 w-1/2 text-left border">"Typical Values"</th>
                        <th class="p-2 w-1/2 text-right border">
                            "Per " {data.data_value} {data.data_measurement.to_string()}
                        </th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td class="p-2 w-1/2 text-left border">"Energy (kcal)"</td>
                        <td class="p-2 w-1/2 text-right border">{data.energy} "kcal"</td>
                    </tr>
                    <tr>
                        <td class="p-2 w-1/2 text-left border">"Protein"</td>
                        <td class="p-2 w-1/2 text-right border">
                            {format!("{:.2}", data.protein)}
                        </td>
                    </tr>
                    <tr>
                        <td class="p-2 w-1/2 text-left border">"Carbohydrate"</td>
                        <td class="p-2 w-1/2 text-right border">
                            {format!("{:.2}", data.carbohydrate)}
                        </td>
                    </tr>
                    <tr>
                        <td class="p-2 w-1/2 text-left border">"Fat"</td>
                        <td class="p-2 w-1/2 text-right border">{format!("{:.2}", data.fat)}</td>
                    </tr>
                    <tr>
                        <td class="p-2 w-1/2 text-left border">"Saturates"</td>
                        <td class="p-2 w-1/2 text-right border">
                            {format!("{:.2}", data.saturates)}
                        </td>
                    </tr>
                    <tr>
                        <td class="p-2 w-1/2 text-left border">"Sugars"</td>
                        <td class="p-2 w-1/2 text-right border">{format!("{:.2}", data.sugars)}</td>
                    </tr>
                    <tr>
                        <td class="p-2 w-1/2 text-left border">"Fibre"</td>
                        <td class="p-2 w-1/2 text-right border">{format!("{:.2}", data.fibre)}</td>
                    </tr>
                    <tr>
                        <td class="p-2 w-1/2 text-left border">"Salt"</td>
                        <td class="p-2 w-1/2 text-right border">{format!("{:.2}", data.salt)}</td>
                    </tr>
                    <tr>
                        <td class="p-2 w-1/2 text-left border">"Protein %"</td>
                        <td class="p-2 w-1/2 text-right border">
                            {format!("{:.2}%", data.protein_pct)}
                        </td>
                    </tr>
                    <tr>
                        <td class="p-2 w-1/2 text-left border">"Carbohydrate %"</td>
                        <td class="p-2 w-1/2 text-right border">
                            {format!("{:.2}%", data.carbohydrate_pct)}
                        </td>
                    </tr>
                    <tr>
                        <td class="p-2 w-1/2 text-left border">"Fat %"</td>
                        <td class="p-2 w-1/2 text-right border">
                            {format!("{:.2}%", data.fat_pct)}
                        </td>
                    </tr>
                    <tr>
                        <td class="p-2 w-1/2 text-left border">"Created"</td>
                        <td class="p-2 w-1/2 text-right border">{created_at}</td>
                    </tr>
                    <tr>
                        <td class="p-2 w-1/2 text-left border">"Updated"</td>
                        <td class="p-2 w-1/2 text-right border">{updated_at}</td>
                    </tr>
                </tbody>
            </table>
        </div>
        <UpdateDeleteButtonRow/>
    }
}
