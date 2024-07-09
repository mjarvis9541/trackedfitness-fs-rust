use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use std::collections::HashSet;

use crate::component::checkbox::CheckboxListItem;
use crate::component::input::FilterInput;
use crate::component::select::FilterSelect;
use crate::component::template::{
    AutoListHeader, ErrorComponent, ListLoadingComponent, ListNotFoundComponent,
};
use crate::user_statistic::model::UserStatistic;
use crate::util::param::{extract_page, extract_param, extract_size};

#[server]
pub async fn get_admin_user_stat_list(
    search: String,
    order: String,
    size: i64,
    page: i64,
) -> Result<Vec<UserStatistic>, ServerFnError> {
    crate::auth::service::extract_superuser_from_request()?;
    let pool = crate::setup::get_pool()?;
    let results = UserStatistic::filter(&pool, &search, &order, size, page).await?;
    Ok(results)
}

#[component]
pub fn AdminUserStatListPage() -> impl IntoView {
    let query = use_query_map();
    let search = move || extract_param(&query, "search");
    let order = move || extract_param(&query, "order");
    let size = move || extract_size(&query);
    let page = move || extract_page(&query);

    let resource = Resource::new(
        move || (search(), order(), size(), page()),
        |(search, order, size, page)| get_admin_user_stat_list(search, order, size, page),
    );

    let all_items = RwSignal::new(HashSet::<String>::new());
    let checked_items = RwSignal::new(HashSet::<String>::new());

    let response = move || {
        resource.and_then(|data| {
            if data.is_empty() {
                view! { <ListNotFoundComponent/> }
            } else {
                let ids: HashSet<String> = data.iter().map(|item| item.id.to_string()).collect();
                all_items.update(|set| set.extend(ids));
                data.iter()
                    .map(|data| {
                        view! { <AdminUserStatListItem data=data.clone() checked_items/> }
                    })
                    .collect_view()
            }
        })
    };

    let sort_options = vec![
        ("username", "Username (A-z)"),
        ("-username", "Username (Z-a)"),
        ("-follower_count", "Follower Count (Desc)"),
        ("follower_count", "Follower Count (Asc)"),
        ("-following_count", "Following Count (Desc)"),
        ("following_count", "Following Count (Asc)"),
        ("-diet_count", "Diet Count (Desc)"),
        ("diet_count", "Diet Count (Asc)"),
        ("-diet_day_log_count", "Diet Day Log Count (Desc)"),
        ("diet_day_log_count", "Diet Day Log Count (Asc)"),
        ("-diet_target_count", "Diet Target Count (Desc)"),
        ("diet_target_count", "Diet Target Count (Asc)"),
        ("-progress_count", "Progress Count (Desc)"),
        ("progress_count", "Progress Count (Asc)"),
        ("-workout_count", "Workout Count (Desc)"),
        ("workout_count", "Workout Count (Asc)"),
        ("-workout_day_log_count", "Workout Day Log Count (Desc)"),
        ("workout_day_log_count", "Workout Day Log Count (Asc)"),
        ("-exercise_count", "Exercise Count (Desc)"),
        ("exercise_count", "Exercise Count (Asc)"),
        ("-set_count", "Set Count (Desc)"),
        ("set_count", "Set Count (Asc)"),
        ("-rep_count", "Rep Count (Desc)"),
        ("rep_count", "Rep Count (Asc)"),
        ("-food_created_count", "Food Created Count (Desc)"),
        ("food_created_count", "Food Created Count (Asc)"),
        ("-brand_created_count", "Brand Created Count (Desc)"),
        ("brand_created_count", "Brand Created Count (Asc)"),
        ("-meal_created_count", "Meal Created Count (Desc)"),
        ("meal_created_count", "Meal Created Count (Asc)"),
        ("-meal_food_created_count", "Meal Food Created Count (Desc)"),
        ("meal_food_created_count", "Meal Food Created Count (Asc)"),
        (
            "-meal_of_day_created_count",
            "Meal Of Day Created Count (Desc)",
        ),
        (
            "meal_of_day_created_count",
            "Meal Of Day Created Count (Asc)",
        ),
        ("-movement_created_count", "Movement Created Count (Desc)"),
        ("movement_created_count", "Movement Created Count (Asc)"),
        (
            "-muscle_group_created_count",
            "Muscle Group Created Count (Desc)",
        ),
        (
            "muscle_group_created_count",
            "Muscle Group Created Count (Asc)",
        ),
    ];

    view! {
        <Title text="Admin - User Stats"/>
        <main class="lg:p-4">
            <div class="p-4 bg-white border">
                <h2 class="mb-4 text-base font-bold">"Admin - User Stats"</h2>
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

                <section class="grid overflow-auto mb-4 whitespace-nowrap bg-white grid-cols-checkbox-12">
                    <AutoListHeader all_items checked_items>
                        "Username"
                        "Followers"
                        "Following"
                        "Diet Logs"
                        "Diet Targets"
                        "Progress"
                        "Workouts"
                        "Exercises"
                        "Sets"
                        "Reps"
                        "Food"
                        "Meals"
                    </AutoListHeader>
                    <Transition fallback=ListLoadingComponent>
                        <ErrorBoundary fallback=|errors| {
                            view! { <ErrorComponent errors=errors/> }
                        }>{response}</ErrorBoundary>
                    </Transition>

                </section>

            </div>
        </main>
    }
}

#[component]
pub fn AdminUserStatListItem(
    data: UserStatistic,
    checked_items: RwSignal<HashSet<String>>,
) -> impl IntoView {
    view! {
        <div class="contents group">
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50 truncate">
                <CheckboxListItem id=data.id.to_string() checked_items/>
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50 truncate">
                <A href=format!("/admin/users/{}", &data.id) class="hover:underline">
                    {data.username}
                </A>
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50 truncate">
                {data.follower_count}
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50 truncate">
                {data.following_count}
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50 truncate">
                {data.diet_count}
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50 truncate">
                {data.diet_target_count}
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50 truncate">
                {data.progress_count}
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50 truncate">
                {data.workout_count}
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50 truncate">
                {data.exercise_count}
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50 truncate">
                {data.set_count}
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50 truncate">
                {data.rep_count}
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50 truncate">
                {data.food_created_count}
            </div>
            <div class="flex items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50 truncate">
                {data.meal_created_count}
            </div>
        </div>
    }
}
