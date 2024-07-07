use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use chrono::prelude::*;
use uuid::Uuid;

use crate::component::button::Button;
use crate::component::icon::IconFilePlus;
use crate::component::input::FilterInput;
use crate::component::paginator::Paginator;
use crate::component::select::FilterSelect;
use crate::component::template::{
    AddFoodListHeader, ErrorComponent, FoodListItemMacroHeader, ListNotFoundComponent,
    ListPageHeaderWithCreate, Skeleton,
};
use crate::food::nutrition_row::NutritionRow;
use crate::meal::model::Meal;
use crate::util::datetime::DATE_FORMAT_SHORT;
use crate::util::misc::ListResponse;
use crate::util::param::{extract_page, extract_param, extract_size, get_date, get_username};

#[cfg(feature = "ssr")]
use crate::{
    auth::model::User, auth::service::get_request_user, diet::model::Diet, error::Error,
    meal_food::model::MealFoodModel, meal_of_day::model::MealOfDay, setup::get_pool,
};

#[server]
pub async fn diet_add_meal(
    username: String,
    date: NaiveDate,
    meal_of_day_slug: String,
    meal_id: Uuid,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    let target_user = User::get_by_username(&pool, &username)
        .await?
        .ok_or(Error::NotFound)?;
    Diet::can_create(&user, user.id)?;
    let meal_of_day = MealOfDay::get_by_slug(&pool, &meal_of_day_slug)
        .await?
        .ok_or(Error::NotFound)?;
    let meal_food = MealFoodModel::all_by_meal_id(&pool, meal_id).await?;
    Diet::bulk_create_from_meal_food(
        &pool,
        target_user.id,
        date,
        meal_of_day.id,
        &meal_food,
        user.id,
    )
    .await?;
    leptos_axum::redirect(&format!("/users/{}/diet/{}", user.username, date));
    Ok(())
}

#[server]
pub async fn get_diet_add_meal_list(
    username: String,
    search: String,
    order: String,
    size: i64,
    page: i64,
) -> Result<ListResponse<Meal>, ServerFnError> {
    let _user = get_request_user()?;
    let pool = get_pool()?;
    let count = Meal::count(&pool, &username, &search).await?;
    let results = Meal::filter(&pool, &username, &search, &order, size, page).await?;
    Ok(ListResponse { count, results })
}

#[component]
pub fn DietAddMealPage() -> impl IntoView {
    let action = Action::<DietAddMeal, _>::server();

    let params = use_params_map();
    let username = move || get_username(&params);
    let date = move || get_date(&params);
    let meal = move || extract_param(&params, "meal");

    let query = use_query_map();
    let search = move || extract_param(&query, "search");
    let order = move || extract_param(&query, "order");
    let size = move || extract_size(&query);
    let page = move || extract_page(&query);

    let resource = Resource::new(
        move || (username(), search(), order(), size(), page()),
        |(username, search, order, size, page)| {
            get_diet_add_meal_list(username, search, order, size, page)
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
                            <DietAddMealListItem
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
    let subtitle = move || format!("{} - {}", date().format(DATE_FORMAT_SHORT), meal());
    let sort_options = vec![
        ("name", "Name (A-z)"),
        ("-name", "Name (Z-a)"),
        ("-food_count", "Food Count (High-Low)"),
        ("food_count", "Food Count (Low-High)"),
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
        <Title text="Add Meal to Diet"/>
        <main class="p-4 space-y-4 bg-white border md:m-4">
            <ListPageHeaderWithCreate
                title="Add Meal to Diet Log"
                subtitle=Signal::derive(subtitle)
                create_href="/food/meals/create"
            >
                <Transition>{count}</Transition>
            </ListPageHeaderWithCreate>

            <section class="flex flex-wrap gap-2 mb-4 lg:mb-2">
                <Form method="GET" action="" class="contents">
                    <input type="hidden" name="size" value=size/>
                    <input type="hidden" name="page" value=1/>
                    <FilterInput name="search" value=Signal::derive(search)/>
                    <FilterSelect name="order" value=Signal::derive(order) options=sort_options/>
                </Form>
            </section>
            <section class="grid grid-cols-4 mb-4 lg:grid-cols-input-12">
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
                    <input type="hidden" name="order" value=order/>
                    <input type="hidden" name="page" value=1/>
                    <Transition>
                        <Paginator count/>
                    </Transition>
                </Form>
            </section>
        </main>
    }
}

#[component]
fn DietAddMealListItem(
    username: String,
    date: NaiveDate,
    meal: String,
    data: Meal,
    action: Action<DietAddMeal, Result<(), ServerFnError>>,
) -> impl IntoView {
    let detail_href = data.get_detail_href();
    let nutrition = data.nutrition.clone();
    view! {
        <ActionForm action class="contents group">
            <input type="hidden" name="username" value=username/>
            <input type="hidden" name="date" value=date.to_string()/>
            <input type="hidden" name="meal_of_day_slug" value=meal/>
            <input type="hidden" name="meal_id" value=data.id.to_string()/>
            <div class="flex col-span-3 items-center py-1 px-2 lg:col-span-2 group-hover:bg-gray-200 group-odd:bg-gray-50 truncate">
                <div>
                    <A class="block font-bold md:font-normal hover:underline" href=detail_href>
                        {data.name}
                    </A>
                    <A
                        class="block text-xs capitalize hover:underline"
                        href=format!("/users/{}", data.username)
                    >
                        {data.username}
                    </A>
                </div>
            </div>
            <div class="flex justify-end items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
                {data.food_count}
            </div>
            <FoodListItemMacroHeader/>
            <NutritionRow data=&nutrition/>
            <div class="flex col-span-4 justify-end items-center mb-2 lg:col-span-1 lg:mb-0 group-hover:bg-gray-200 group-odd:bg-gray-50">
                <Button
                    label="Add"
                    loading=action.pending()
                    disabled=Signal::derive(move || !data.food_count > 0)
                >
                    <IconFilePlus/>
                </Button>
            </div>
        </ActionForm>
    }
}
