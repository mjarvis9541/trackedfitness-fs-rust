use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use std::collections::HashSet;

use crate::component::checkbox::CheckboxListItem;
use crate::component::select::USER_STAT_SORT_OPTIONS;
use crate::component::template::{
    AutoListHeader, ErrorComponent, ListLoadingComponent, ListNotFoundComponent, SearchForm,
};
use crate::user_statistic::model::UserStat;
use crate::util::param::{extract_page, extract_param, extract_size};

#[server]
pub async fn get_admin_user_stat_list(
    search: String,
    order: String,
    size: i64,
    page: i64,
) -> Result<Vec<UserStat>, ServerFnError> {
    crate::auth::service::extract_superuser_from_request()?;
    let pool = crate::setup::get_pool()?;
    let results = UserStat::filter(&pool, &search, &order, size, page).await?;
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

    view! {
        <Title text="Admin - User Stats"/>
        <main class="lg:p-4">
            <div class="p-4 bg-white border">
                <h2 class="mb-4 text-base font-bold">"Admin - User Stats"</h2>
                <section class="flex flex-wrap gap-2 mb-4 lg:mb-2">
                    <SearchForm
                        search=Signal::derive(search)
                        order=Signal::derive(order)
                        size=Signal::derive(size)
                        page=1
                        options=&USER_STAT_SORT_OPTIONS
                    />
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
    data: UserStat,
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
