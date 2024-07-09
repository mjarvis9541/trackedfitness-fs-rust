use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use rust_decimal::Decimal;
use std::collections::HashSet;

use crate::brand::select::BrandFilter;
use crate::component::bulk_delete::BulkDeleteForm;
use crate::component::checkbox::CheckboxListItem;
use crate::component::input::FilterInput;
use crate::component::paginator::Paginator;
use crate::component::select::FilterSelect;
use crate::component::template::{
    AutoListHeader, ErrorComponent, ListLoadingComponent, ListNotFoundComponent,
    ListPageHeaderWithCreate,
};
use crate::util::misc::ListResponse;
use crate::util::param::{extract_page, extract_param, extract_size};

use super::data_measurement::DataMeasurement;
use super::model::FoodQuery;
use super::nutrition_row_calc::FoodNutritionCalculationRow;

#[cfg(feature = "ssr")]
use crate::{auth::service::get_request_user, setup::get_pool};

#[server]
pub async fn get_food_list(
    search: String,
    brand: String,
    serving: String,
    order: String,
    size: i64,
    page: i64,
) -> Result<ListResponse<FoodQuery>, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    let count = FoodQuery::count(&pool, &search, &brand, &serving).await?;
    let results = FoodQuery::filter(
        &pool,
        &search,
        &brand,
        &serving,
        Some(user.id),
        &order,
        size,
        page,
    )
    .await?;
    Ok(ListResponse { count, results })
}

#[component]
pub fn FoodListPage() -> impl IntoView {
    let action_bulk_delete = Action::server();
    let query = use_query_map();
    let search = move || extract_param(&query, "search");
    let brand = move || extract_param(&query, "brand");
    let serving = move || extract_param(&query, "serving");
    let order = move || extract_param(&query, "order");
    let size = move || extract_size(&query);
    let page = move || extract_page(&query);

    let resource = Resource::new(
        move || {
            (
                search(),
                brand(),
                serving(),
                order(),
                size(),
                page(),
                action_bulk_delete.version().get(),
            )
        },
        |(search, brand, serving, order, size, page, _)| {
            get_food_list(search, brand, serving, order, size, page)
        },
    );

    let all_items = RwSignal::new(HashSet::<String>::new());
    let checked_items: RwSignal<HashSet<String>> = RwSignal::new(HashSet::<String>::new());

    let response = move || {
        resource.and_then(|data| {
            let count = data.count;
            let results = &data.results;
            if count == 0 {
                view! { <ListNotFoundComponent/> }
            } else {
                let ids: HashSet<String> = results.iter().map(|item| item.id.to_string()).collect();
                all_items.update(|set| set.extend(ids));
                results
                    .iter()
                    .map(|data| {
                        view! { <FoodListItem data=data.clone() checked_items/> }
                    })
                    .collect_view()
            }
        })
    };
    let count = move || {
        resource.with(|res| {
            res.as_ref()
                .and_then(|data| data.as_ref().ok().map(|res| res.count))
        })
    };
    let serving_options = DataMeasurement::to_filter_options();
    let sort_options = FoodQuery::to_filter_options();

    view! {
        <Title text="Food"/>
        <main class="p-4 bg-white border md:m-4">
            <ListPageHeaderWithCreate title="Food" create_href="create">
                <Transition>{count}</Transition>
            </ListPageHeaderWithCreate>

            <section class="flex flex-wrap gap-2 mb-4 lg:mb-2">
                <Form method="GET" action="" class="contents">
                    <input type="hidden" name="size" value=size/>
                    <input type="hidden" name="page" value=1/>
                    <FilterInput name="search" value=Signal::derive(search)/>
                    <BrandFilter selected=Signal::derive(brand)/>
                    <FilterSelect
                        name="serving"
                        value=Signal::derive(serving)
                        options=serving_options
                    />
                    <FilterSelect name="order" value=Signal::derive(order) options=sort_options/>
                </Form>
            </section>
            <section class="grid grid-cols-4 lg:grid-cols-checkbox-12">
                <AutoListHeader all_items checked_items>
                    "Food"
                    " "
                    " "
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
                <Transition fallback=ListLoadingComponent>
                    <ErrorBoundary fallback=|errors| {
                        view! { <ErrorComponent errors/> }
                    }>{response}</ErrorBoundary>
                </Transition>
            </section>
            <section class="flex flex-wrap pt-4">
                <div class="hidden md:block">
                    <BulkDeleteForm table="food" action=action_bulk_delete checked_items/>
                </div>

                <div class="flex-1">
                    <Form method="GET" action="" class="contents">
                        <input type="hidden" name="page" value=page/>
                        <input type="hidden" name="search" value=search/>
                        <input type="hidden" name="brand" value=brand/>
                        <input type="hidden" name="serving" value=serving/>
                        <input type="hidden" name="order" value=order/>
                        <Transition>
                            <Paginator count/>
                        </Transition>
                    </Form>
                </div>
            </section>

        </main>
    }
}

#[component]
pub fn FoodListItem(data: FoodQuery, checked_items: RwSignal<HashSet<String>>) -> impl IntoView {
    let data_value_decimal = Decimal::from(data.data_value);
    let quantity: RwSignal<Decimal> = RwSignal::new(data_value_decimal);
    view! {
        <div class="contents group">
            <div class="hidden justify-center items-center py-2 px-2 lg:flex group-hover:bg-gray-200 group-odd:bg-gray-50">
                <CheckboxListItem id=data.id.to_string() checked_items/>
            </div>
            <FoodNutritionCalculationRow data data_value_decimal quantity/>
        </div>
    }
}
