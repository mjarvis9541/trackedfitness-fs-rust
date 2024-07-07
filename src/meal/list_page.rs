use std::collections::HashSet;

use leptos::server_fn::codec::GetUrl;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use super::model::Meal;
use crate::component::bulk_delete::BulkDeleteForm;
use crate::component::checkbox::CheckboxListItem;
use crate::component::input::FilterInput;
use crate::component::paginator::Paginator;
use crate::component::select::FilterSelect;
use crate::component::template::{
    AutoListHeader, ErrorComponent, FoodListItemMacroHeader, ListNotFoundComponent,
    ListPageHeaderWithCreate, Skeleton,
};
use crate::food::nutrition_row::NutritionRow;
use crate::util::misc::ListResponse;
use crate::util::param::{extract_page, extract_param, extract_size};

#[cfg(feature = "ssr")]
use crate::auth::service::get_request_user;

#[server(endpoint = "meal-list", input = GetUrl)]
pub async fn get_meal_list(
    search: String,
    order: String,
    size: i64,
    page: i64,
) -> Result<ListResponse<Meal>, ServerFnError> {
    get_request_user()?;
    let pool = expect_context::<sqlx::PgPool>();
    let username = String::from("");
    let count = Meal::count(&pool, &username, &search).await?;
    let results = Meal::filter(&pool, &username, &search, &order, size, page).await?;
    Ok(ListResponse { count, results })
}

#[component]
pub fn MealListPage() -> impl IntoView {
    let action_bulk_delete = Action::server();

    let query = use_query_map();
    let search = move || extract_param(&query, "search");
    let order = move || extract_param(&query, "order");
    let size = move || extract_size(&query);
    let page = move || extract_page(&query);

    let resource = Resource::new(
        move || {
            (
                search(),
                order(),
                size(),
                page(),
                action_bulk_delete.version().get(),
            )
        },
        |(search, order, size, page, _)| get_meal_list(search, order, size, page),
    );

    let all_items = RwSignal::new(HashSet::<String>::new());
    let checked_items = RwSignal::new(HashSet::<String>::new());

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
                        view! { <MealListItem data=data.clone() checked_items/> }
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
        <Title text="Meals"/>
        <main class="md:p-4">

            <div class="p-4 bg-white border">
                <ListPageHeaderWithCreate title="Meals" create_href="create">
                    <Transition>{count}</Transition>
                </ListPageHeaderWithCreate>

                <section class="flex flex-wrap gap-2 mb-4 lg:mb-2">
                    <Form method="GET" action="" class="contents">
                        <input type="hidden" name="size" value=size/>
                        <input type="hidden" name="page" value=1/>
                        <FilterInput name="search" value=Signal::derive(search)/>
                        <FilterSelect
                            name="order"
                            value=Signal::derive(order)
                            options=sort_options
                        />
                    </Form>
                </section>

                <section class="grid grid-cols-4 lg:grid-cols-checkbox-12">
                    <AutoListHeader all_items checked_items>
                        "Meal"
                        " "
                        " "
                        "Food"
                        "Calories"
                        "Protein"
                        "Carbs"
                        "Fat"
                        "Sat.Fat"
                        "Sugars"
                        "Fibre"
                        "Salt"
                    </AutoListHeader>
                    <Transition fallback=|| view! { <Skeleton row_count=25/> }>
                        <ErrorBoundary fallback=|errors| {
                            view! { <ErrorComponent errors/> }
                        }>{response}</ErrorBoundary>
                    </Transition>
                </section>

                <section class="flex flex-wrap pt-4">
                    <div class="hidden md:block">
                        <BulkDeleteForm table="meal" action=action_bulk_delete checked_items/>
                    </div>
                    <div class="flex-1">
                        <Form method="GET" action="" class="contents">
                            <input type="hidden" name="search" value=search/>
                            <input type="hidden" name="order" value=order/>
                            <input type="hidden" name="page" value=page/>
                            <Transition>
                                <Paginator count/>
                            </Transition>
                        </Form>
                    </div>
                </section>
            </div>
        </main>
    }
}

#[component]
pub fn MealListItem(data: Meal, checked_items: RwSignal<HashSet<String>>) -> impl IntoView {
    let nutrition = &data.nutrition;
    view! {
        <div class="contents group">
            <div class="hidden justify-center items-center py-2 px-2 lg:flex group-hover:bg-gray-200 group-odd:bg-gray-50">
                <CheckboxListItem id=data.id.to_string() checked_items/>
            </div>
            <div class="flex flex-col col-span-3 p-1 px-2 md:col-span-3 group-hover:bg-gray-200 group-odd:bg-gray-50 truncate">
                <A class="font-bold md:font-normal hover:underline" href=data.id.to_string()>
                    {data.name}
                </A>
                <A class="text-sm hover:underline" href=format!("/users/{}", data.username)>
                    {data.username}
                </A>
            </div>
            <div class="flex justify-end items-center py-2 px-2 lg:mb-0 group-hover:bg-gray-200 group-odd:bg-gray-50">
                <A class="hover:underline" href=format!("{}/update", data.id)>
                    {data.food_count}
                </A>
            </div>
            <FoodListItemMacroHeader/>
            <NutritionRow data=nutrition/>
        </div>
    }
}
