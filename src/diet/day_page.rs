use std::collections::HashSet;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::component::bulk_delete::BulkDeleteForm;
use crate::component::button::Button;
use crate::component::checkbox::{CheckboxListHeader, CheckboxListItem};
use crate::component::date_navigation::DateNavigation;
use crate::component::icon::IconFilePlus;
use crate::component::template::{ErrorComponent, LoadingSpinner};
use crate::diet_target::model::DietTargetQuery;
use crate::food::model::Nutrition;
use crate::util::param::{get_date, get_username};

use super::component::{DietFoodGridHeader, DietMealGridHeader};
use super::copy_previous_day_form::{DietCopyPreviousDay, DietCopyPreviousDayForm};
use super::copy_previous_meal_form::DietCopyPrevious;
use super::model::{DietDayDTO, DietFoodQuery, DietMealDTO, FormattedFoodData};
use super::to_meal_form::{DietToMealForm, SaveToMeal};
use super::week_navigation::DietWeekNavComponent;

#[cfg(feature = "ssr")]
use crate::{
    auth::model::User, auth::service::get_request_user, diet::service::DietService, setup::get_pool,
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DietDayResponse {
    pub diet_day: DietDayDTO,
    pub diet_target: Option<DietTargetQuery>,
    pub remaining: Option<Nutrition>,
}

#[server(endpoint = "get-diet-day")]
async fn get_diet_day(username: String, date: NaiveDate) -> Result<DietDayResponse, ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    User::check_view_permission(&pool, &user, &username).await?;
    let response = DietService::aggregate_diet_day_data(&pool, &username, date).await?;
    Ok(response)
}

#[component]
pub fn DietDayPage() -> impl IntoView {
    let action_diet_bulk_delete = Action::server();
    let action_diet_save_to_meal = Action::<SaveToMeal, _>::server();
    let action_diet_copy_previous = Action::<DietCopyPrevious, _>::server();
    let action_diet_copy_previous_day = Action::<DietCopyPreviousDay, _>::server();

    provide_context(action_diet_save_to_meal);
    provide_context(action_diet_copy_previous);

    let params = use_params_map();
    let username = move || get_username(&params);
    let date = move || get_date(&params);

    let resource = Resource::new(
        move || {
            (
                username(),
                date(),
                action_diet_bulk_delete.version().get(),
                action_diet_save_to_meal.version().get(),
                action_diet_copy_previous.version().get(),
                action_diet_copy_previous_day.version().get(),
            )
        },
        |(username, date, ..)| get_diet_day(username, date),
    );

    let checked_items = RwSignal::new(HashSet::<String>::new());

    let response = move || {
        resource.and_then(|data| {
            let data = data.clone();
            let diet_data = data.diet_day;
            let target_view = data.diet_target.map(|target| {
                let total = target.format();
                view! {
                    <section class="flex col-span-4 items-center p-2 font-bold bg-gray-200">
                        "Target"
                    </section>
                    <DietDayTotalComponent formatted=total/>
                }
            });
            let remain_view = data.remaining.map(|remaining| {
                let total = remaining.format();
                view! {
                    <section class="flex col-span-4 items-center p-2 font-bold bg-gray-200">
                        "Remaining"
                    </section>
                    <DietDayTotalComponent formatted=total/>
                }
            });
            view! {
                <DietDayComponent data=diet_data checked_items/>
                {target_view}
                {remain_view}
            }
        })
    };

    let show_save_modal = RwSignal::new(false);
    let toggle_save_modal = move |_| show_save_modal.update(|value| *value = !*value);
    let disabled_save_modal = Signal::derive(move || checked_items.with(HashSet::is_empty));

    view! {
        <Title text="Diet"/>
        <main class="p-4">
            <DateNavigation/>
            <DietWeekNavComponent/>
            <section class="grid grid-cols-4 lg:grid-cols-12">
                <Transition fallback=LoadingSpinner>
                    <ErrorBoundary fallback=|errors| {
                        view! { <ErrorComponent errors/> }
                    }>{response}</ErrorBoundary>
                </Transition>
            </section>
            <section class="flex flex-wrap gap-x-2 p-2 mt-2 bg-gray-200">
                <div class="hidden lg:block">
                    <BulkDeleteForm table="food_log" action=action_diet_bulk_delete checked_items/>
                </div>
                <div>
                    <Button
                        label="Save as Meal"
                        disabled=disabled_save_modal
                        on:click=toggle_save_modal
                    >
                        <IconFilePlus/>
                    </Button>
                </div>
                <DietCopyPreviousDayForm action=action_diet_copy_previous_day/>
            </section>
            <section class="max-w-sm border p-4 m-4">
                <DietToMealForm action=action_diet_save_to_meal checked_items/>
            </section>
        </main>
    }
}

const DAY_TOTAL_ROW_CSS: &str = "flex items-center justify-end p-2 bg-gray-200 text-xs font-bold";
const SECONDARY_DAY_TOTAL_ROW_CSS: &str =
    "hidden md:flex items-center justify-end p-2 bg-gray-200 text-xs font-bold";

#[component]
pub fn DietDayComponent(
    data: DietDayDTO,
    checked_items: RwSignal<HashSet<String>>,
) -> impl IntoView {
    let total = data.format();
    let meal_list = data.meal_list;
    let diet_meal_list_view = meal_list
        .into_iter()
        .map(|meal| {
            view! { <DietMealComponent data=meal checked_items/> }
        })
        .collect_view();
    view! {
        {diet_meal_list_view}
        <div class="col-span-full h-2 bg-gray-100 lg:hidden"></div>
        <section class="flex col-span-4 items-center p-2 font-bold bg-gray-200">"Total"</section>
        <DietDayTotalComponent formatted=total/>
    }
}

#[component]
pub fn DietDayTotalComponent(formatted: FormattedFoodData) -> impl IntoView {
    view! {
        <div class=DAY_TOTAL_ROW_CSS>{formatted.energy}</div>
        <div class=DAY_TOTAL_ROW_CSS>{formatted.protein}" "{formatted.protein_pct}</div>
        <div class=DAY_TOTAL_ROW_CSS>{formatted.carbohydrate}" "{formatted.carbohydrate_pct}</div>
        <div class=DAY_TOTAL_ROW_CSS>{formatted.fat}" "{formatted.fat_pct}</div>
        <div class=SECONDARY_DAY_TOTAL_ROW_CSS>{formatted.saturates}</div>
        <div class=SECONDARY_DAY_TOTAL_ROW_CSS>{formatted.sugars}</div>
        <div class=SECONDARY_DAY_TOTAL_ROW_CSS>{formatted.fibre}</div>
        <div class=SECONDARY_DAY_TOTAL_ROW_CSS>{formatted.salt}</div>
    }
}

const MEAL_TOTAL_ROW_CSS: &str =
    "flex items-center justify-end p-2 bg-gray-200/75 font-bold text-xs";
const SECONDARY_MEAL_TOTAL_ROW_CSS: &str =
    "hidden md:flex items-center justify-end p-2 bg-gray-200/75 font-bold text-xs";

#[component]
pub fn DietMealComponent(
    data: DietMealDTO,
    checked_items: RwSignal<HashSet<String>>,
) -> impl IntoView {
    let total = data.format();
    let add_food_url = data.diet_add_food_url();
    let add_meal_url = data.diet_add_meal_url();

    let food_list = data.food_list;
    let all_items = RwSignal::new(DietFoodQuery::ids_as_set(&food_list));

    let meal_name = data.name;
    let meal_order = data.ordering;

    let diet_food_list_view = food_list
        .into_iter()
        .map(|diet_food| {
            view! { <DietFoodComponent data=diet_food checked_items/> }
        })
        .collect_view();

    view! {
        <header class="col-span-4 flex flex-wrap items-start gap-2 border-b py-1 lg:col-span-3">
            <div class="flex flex-1 gap-2 items-center">
                <div class="p-2 hidden items-center justify-center lg:flex">
                    <CheckboxListHeader all_items checked_items/>
                </div>
                <div class="flex-1">
                    <h2 class="text-base font-bold">{meal_name}</h2>
                    <p class="text-xs text-gray-500">"Meal " {meal_order}</p>
                </div>
            </div>
        </header>
        <DietMealGridHeader/>
        {diet_food_list_view}
        <div class="flex justify-between p-2 bg-gray-200/75 col-span-4">
            <div class="font-bold">"Total"</div>
            <div class="flex gap-2">
                <a class="hover:underline font-semibold" href=&add_food_url>
                    "Add Food"
                </a>
                <a class="hover:underline font-semibold" href=&add_meal_url>
                    "Add Meal"
                </a>
            </div>
        </div>
        <div class=MEAL_TOTAL_ROW_CSS>{total.energy}</div>
        <div class=MEAL_TOTAL_ROW_CSS>{total.protein}" "{total.protein_pct}</div>
        <div class=MEAL_TOTAL_ROW_CSS>{total.carbohydrate}" "{total.carbohydrate_pct}</div>
        <div class=MEAL_TOTAL_ROW_CSS>{total.fat}" "{total.fat_pct}</div>
        <div class=SECONDARY_MEAL_TOTAL_ROW_CSS>{total.saturates}</div>
        <div class=SECONDARY_MEAL_TOTAL_ROW_CSS>{total.sugars}</div>
        <div class=SECONDARY_MEAL_TOTAL_ROW_CSS>{total.fibre}</div>
        <div class=SECONDARY_MEAL_TOTAL_ROW_CSS>{total.salt}</div>
        <section class="col-span-full h-2"></section>
    }
}

const TITLE_ROW_CSS: &str =
    "col-span-4 flex items-center gap-2 py-1 pr-2 group-odd:bg-gray-50 group-hover:bg-amber-200";
const ROW_CSS: &str =
    "flex items-center justify-end p-2 group-odd:bg-gray-50 group-hover:bg-amber-200";
const SECONDARY_ROW_CSS: &str =
    "hidden md:flex items-center justify-end p-2 group-odd:bg-gray-50 group-hover:bg-amber-200";

#[component]
pub fn DietFoodComponent(
    data: DietFoodQuery,
    checked_items: RwSignal<HashSet<String>>,
) -> impl IntoView {
    let formatted = data.format();
    let detail_url = data.diet_detail_url();
    let update_url = data.diet_update_url();
    let brand_url = data.brand_detail_url();

    let serving = data.get_serving_display();
    let food_name = data.food_name;
    let brand_name = data.brand_name;

    let diet_id = data.id.to_string();

    view! {
        <section class="contents group">
            <div class=TITLE_ROW_CSS>
                <div class="hidden p-2 lg:block">
                    <CheckboxListItem id=diet_id checked_items/>
                </div>
                <div class="flex-1">
                    <a href=detail_url class="block font-bold md:font-normal">
                        {food_name}
                    </a>
                    <a href=brand_url class="block text-xs text-gray-600 capitalize">
                        {brand_name}
                    </a>
                </div>
                <div class="flex justify-end items-center">
                    <a href=update_url class="font-bold md:font-normal hover:underline">
                        {serving}
                    </a>
                </div>
            </div>
            <DietFoodGridHeader/>
            <div class=ROW_CSS>{formatted.energy}</div>
            <div class=ROW_CSS>{formatted.protein}</div>
            <div class=ROW_CSS>{formatted.carbohydrate}</div>
            <div class=ROW_CSS>{formatted.fat}</div>
            <div class=SECONDARY_ROW_CSS>{formatted.saturates}</div>
            <div class=SECONDARY_ROW_CSS>{formatted.sugars}</div>
            <div class=SECONDARY_ROW_CSS>{formatted.fibre}</div>
            <div class=SECONDARY_ROW_CSS>{formatted.salt}</div>
        </section>
    }
}
