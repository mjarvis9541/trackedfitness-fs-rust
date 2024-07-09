use leptos::*;
use leptos_router::*;

use rust_decimal::Decimal;
use uuid::Uuid;

use crate::brand::select::BrandFilter;
use crate::component::button::Button;
use crate::component::icon::IconFilePlus;
use crate::component::input::FilterInput;
use crate::component::modal::ErrorModal;
use crate::component::paginator::Paginator;
use crate::component::select::FilterSelect;
use crate::component::template::{
    AddFoodListHeader, ErrorComponent, ListNotFoundComponent, ListPageHeaderWithCreate, Loading,
    Skeleton,
};
use crate::diet::add_food_page::get_add_food_list;
use crate::food::data_measurement::DataMeasurement;
use crate::food::model::FoodQuery;
use crate::food::nutrition_row_calc::FoodNutritionCalculationRow;
use crate::util::param::{extract_page, extract_param, extract_size, UuidParam};

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user, error::Error, meal::model::Meal, meal_food::model::MealFood,
    setup::get_pool,
};

#[server(endpoint = "meal-add-food")]
pub async fn meal_add_food(
    meal_id: Uuid,
    food_id: Uuid,
    quantity: Decimal,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    let object = Meal::get_by_id(&pool, meal_id)
        .await?
        .ok_or(Error::NotFound)?;
    object.can_update(&user).await?;
    let food = FoodQuery::get_by_id(&pool, food_id)
        .await?
        .ok_or(Error::NotFound)?;
    let quantity = food.data_measurement.to_quantity_modifier(&quantity);
    MealFood::validate(quantity)?;
    MealFood::create_and_return_meal_id(&pool, object.id, food_id, quantity, user.id).await?;
    Ok(())
}

#[component]
pub fn MealAddFoodComponent() -> impl IntoView {
    let params = use_params::<UuidParam>();
    let id = move || params.with(|p| p.as_ref().map(|p| p.id).unwrap_or_default());

    let action = expect_context::<Action<MealAddFood, Result<(), ServerFnError>>>();

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
                String::from(""),
                search(),
                brand(),
                serving(),
                order(),
                size(),
                page(),
            )
        },
        |(username, search, brand, serving, order, size, page)| {
            get_add_food_list(username, search, brand, serving, order, size, page)
        },
    );
    let response = move || {
        resource.and_then(|data| {
            let count = data.count;
            let results = &data.results;
            if count == 0 {
                view! { <ListNotFoundComponent/> }
            } else {
                results
                    .iter()
                    .map(|data| {
                        view! { <MealAddFoodListItem data=data.clone() meal_id=id() action/> }
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
    let show_error = RwSignal::new(false);
    let error = move || {
        action.value().with(|opt| {
            opt.as_ref().and_then(|res| match res {
                Ok(_) => {
                    show_error.update(|v| *v = false);
                    None
                },
                Err(err) => {
                    show_error.update(|v| *v = true);
                    Some(view! { <ErrorModal title="Error" show=show_error message=err.to_string()/> })
                }
            })
        })
    };
    let serving_options = DataMeasurement::to_filter_options();
    let sort_options = FoodQuery::to_diet_filter_options();
    view! {
        <ListPageHeaderWithCreate title="Add Food to Meal" create_href="/food/create">
            <Transition fallback=Loading>{count}</Transition>
        </ListPageHeaderWithCreate>

        {error}

        <section class="flex flex-wrap gap-2 mb-4 lg:mb-2">
            <Form method="GET" action="" class="contents">
                <input type="hidden" name="size" value=size/>
                <input type="hidden" name="page" value=1/>
                <FilterInput name="search" value=Signal::derive(search)/>
                <BrandFilter selected=Signal::derive(brand)/>
                <FilterSelect name="serving" value=Signal::derive(serving) options=serving_options/>
                <FilterSelect name="order" value=Signal::derive(order) options=sort_options/>
            </Form>
        </section>

        <section class="grid grid-cols-4 mb-4 md:grid-cols-input-12">
            <AddFoodListHeader title="Food" subtitle="Quantity"/>
            <Transition fallback=|| view! { <Skeleton row_count=25/> }>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors/> }
                }>{response}</ErrorBoundary>
            </Transition>
        </section>

        <section>
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
        </section>
    }
}

#[component]
pub fn MealAddFoodListItem(
    meal_id: Uuid,
    data: FoodQuery,
    action: Action<MealAddFood, Result<(), ServerFnError>>,
) -> impl IntoView {
    let data_value_decimal = data.get_last_added_data_value();
    let quantity = RwSignal::new(data_value_decimal);
    let food_id = data.id.to_string();
    let meal_id = meal_id.to_string();
    view! {
        <ActionForm action class="contents group">
            <FoodNutritionCalculationRow data data_value_decimal quantity/>

            <div class="flex col-span-4 justify-end items-center mb-2 lg:col-span-1 lg:mb-0 group-hover:bg-gray-200 group-odd:bg-gray-50">
                <input type="hidden" name="food_id" value=food_id/>
                <input type="hidden" name="meal_id" value=meal_id/>
                <Button
                    label="Add"
                    loading=action.pending()
                    disabled=Signal::derive(move || quantity.with(Decimal::is_zero))
                >
                    <IconFilePlus/>
                </Button>
            </div>
        </ActionForm>
    }
}
