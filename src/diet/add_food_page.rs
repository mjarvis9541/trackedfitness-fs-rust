use leptos::*;
use leptos_meta::*;
use leptos_router::*;

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
use crate::food::model::Food;
use crate::food::nutrition_row_calc::FoodNutritionCalculationRow;
use crate::util::datetime::DATE_FORMAT_LONG;
use crate::util::misc::ListResponse;
use crate::util::param::{extract_page, extract_param, extract_size, get_date, get_username};
use chrono::prelude::*;
use rust_decimal::Decimal;

#[cfg(feature = "ssr")]
use crate::{
    auth::model::User, auth::service::get_request_user, diet::model::Diet, error::Error,
    meal_of_day::model::MealOfDay, setup::get_pool,
};

#[server]
pub async fn get_add_food_list(
    username: String,
    search: String,
    brand: String,
    serving: String,
    order: String,
    size: i64,
    page: i64,
) -> Result<ListResponse<Food>, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    let user_id = if username.is_empty() {
        user.id
    } else {
        User::get_by_username(&pool, &username)
            .await?
            .ok_or(Error::NotFound)?
            .id
    };
    let count = Food::count(&pool, &search, &brand, &serving).await?;
    let results = Food::filter(
        &pool,
        &search,
        &brand,
        &serving,
        Some(user_id),
        &order,
        size,
        page,
    )
    .await?;
    Ok(ListResponse { count, results })
}

#[server(endpoint = "diet-add-food")]
pub async fn diet_add_food(
    username: String,
    date: NaiveDate,
    meal_of_day_slug: String,
    food_slug: String,
    quantity: Decimal,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let target_user = User::get_by_username(&pool, &username)
        .await?
        .ok_or(Error::NotFound)?;

    Diet::can_create(&user, target_user.id)?;

    Diet::validate(date, quantity)?;

    let food = Food::get_by_slug(&pool, &food_slug)
        .await?
        .ok_or(Error::NotFound)?;

    let quantity = food.data_measurement.to_quantity_modifier(&quantity);

    let meal_of_day = MealOfDay::get_by_slug(&pool, &meal_of_day_slug)
        .await?
        .ok_or(Error::NotFound)?;
    Diet::create(
        &pool,
        date,
        user.id,
        meal_of_day.id,
        food.id,
        quantity,
        user.id,
    )
    .await?;

    leptos_axum::redirect(&format!("/users/{}/diet/{}", user.username, date));
    Ok(())
}

#[component]
pub fn DietAddFoodPage() -> impl IntoView {
    let params = use_params_map();
    let username = move || get_username(&params);
    let date = move || get_date(&params);
    let meal = move || extract_param(&params, "meal");

    let query = use_query_map();
    let search = move || extract_param(&query, "search");
    let brand = move || extract_param(&query, "brand");
    let serving = move || extract_param(&query, "serving");
    let order = move || extract_param(&query, "order");
    let size = move || extract_size(&query);
    let page = move || extract_page(&query);

    let action = Action::<DietAddFood, _>::server();

    let resource = Resource::new(
        move || {
            (
                username(),
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
                        view! {
                            <DietAddFoodListItem
                                data=data.clone()
                                username=username()
                                date=date()
                                meal=meal()
                                action
                            />
                        }
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
    let subtitle = move || format!("{} - {}", date().format(DATE_FORMAT_LONG), meal());
    let show_error = RwSignal::new(false);
    // let error = move || {
    //     action.value().with(|res| match res {
    //         Some(Err(err)) => {
    //             show_error.update(|v| *v = true);
    //             Some(view! { <ErrorModal title="Error" show=show_error message=err.to_string()/> })
    //         }
    //         _ => {
    //             show_error.update(|v| *v = false);
    //             None
    //         }
    //     })
    // };
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
    let serving_options = vec![
        ("", "All"),
        ("g", "100g"),
        ("ml", "100ml"),
        ("srv", "1 Serving"),
    ];
    let sort_options = vec![
        ("name", "Food (A-z)"),
        ("-name", "Food (Z-a)"),
        ("brand_name", "Brand (A-z)"),
        ("-brand_name", "Brand (Z-a)"),
        ("-last_added_quantity", "Last Added Quantity (Desc)"),
        ("last_added_quantity", "Last Added Quantity (Asc)"),
        ("-last_added_date", "Last Added Date (Desc)"),
        ("last_added_date", "Last Added Date (Asc)"),
        ("-energy", "Calories (High-Low)"),
        ("energy", "Calories (Low-High)"),
        ("-protein", "Protein (High-Low)"),
        ("protein", "Protein (Low-High)"),
        ("-carbohydrate", "Carbs (High-Low)"),
        ("carbohydrate", "Carbs (Low-High)"),
        ("-fat", "Fat (High-Low)"),
        ("fat", "Fat (Low-High)"),
        ("-saturates", "Saturates (High-Low)"),
        ("saturates", "Saturates (Low-High)"),
        ("-sugars", "Sugars (High-Low)"),
        ("sugars", "Sugars (Low-High)"),
        ("-fibre", "Fibre (High-Low)"),
        ("fibre", "Fibre (Low-High)"),
        ("-salt", "Salt (High-Low)"),
        ("salt", "Salt (Low-High)"),
        ("-created_at", "Created (Desc)"),
        ("created_at", "Created (Asc)"),
        ("-updated_at", "Updated (Desc)"),
        ("updated_at", "Updated (Asc)"),
    ];
    view! {
        <Title text="Add Food to Diet"/>
        <main class="p-4 space-y-4 bg-white border md:m-4">

            <ListPageHeaderWithCreate title="Add Food to Diet" create_href="/food/create">
                <Transition fallback=Loading>{count}</Transition>
            </ListPageHeaderWithCreate>
            <h2 class="font-bold">{subtitle}</h2>
            {error}
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

            <section class="grid grid-cols-4 lg:grid-cols-input-12">
                <AddFoodListHeader title="Food" subtitle="Quantity"/>
                <Transition fallback=|| view! { <Skeleton row_count=25/> }>
                    <ErrorBoundary fallback=|errors| {
                        view! { <ErrorComponent errors/> }
                    }>{response}</ErrorBoundary>
                </Transition>
            </section>

            <section>
                <Form method="GET" action="" class="contents">
                    <input type="hidden" name="search" value=search/>
                    <input type="hidden" name="brand" value=brand/>
                    <input type="hidden" name="serving" value=serving/>
                    <input type="hidden" name="order" value=order/>
                    <input type="hidden" name="page" value=page/>
                    <Transition>
                        <Paginator count/>
                    </Transition>
                </Form>
            </section>

        </main>
    }
}

#[component]
pub fn DietAddFoodListItem(
    username: String,
    date: NaiveDate,
    meal: String,
    data: Food,
    action: Action<DietAddFood, Result<(), ServerFnError>>,
) -> impl IntoView {
    let data_value_decimal = data.get_last_added_data_value();
    let quantity = RwSignal::new(data_value_decimal);
    let slug = data.slug.clone();
    let date = date.to_string();
    view! {
        <ActionForm action class="contents group">
            <FoodNutritionCalculationRow data data_value_decimal=data_value_decimal quantity/>

            <div class="flex col-span-4 justify-end items-center mb-2 lg:col-span-1 lg:mb-0 group-hover:bg-gray-200 group-odd:bg-gray-50">
                <input type="hidden" name="username" value=username/>
                <input type="hidden" name="date" value=date/>
                <input type="hidden" name="meal_of_day_slug" value=meal/>
                <input type="hidden" name="food_slug" value=slug/>
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
