use leptos::*;
use leptos_router::*;

use chrono::NaiveDate;

use crate::diet::add_food_page::DietAddFoodPage;
use crate::diet::add_meal_page::DietAddMealPage;
use crate::diet::day_page::DietDayPage;
use crate::diet::delete_page::DietDeletePage;
use crate::diet::detail_page::DietDetailPage;
use crate::diet::update_page::DietUpdatePage;

#[derive(Debug, PartialEq, Params)]
pub struct DietMealParam {
    pub username: String,
    pub date: NaiveDate,
    pub meal: String,
}

#[derive(Debug, PartialEq, Params)]
pub struct DietDetailParam {
    pub username: String,
    pub date: NaiveDate,
    pub meal: String,
}

#[component(transparent)]
pub fn DietRouter() -> impl IntoView {
    view! {
        <Route path="/diet" view=DietLayout>
            <Route path="/:date?" view=DietDayPage/>
            <Route path="/:date/:meal/add-food" view=DietAddFoodPage/>
            <Route path="/:date/:meal/add-meal" view=DietAddMealPage/>
            <Route path="/:date/:meal/:id" view=DietDetailPage/>
            <Route path="/:date/:meal/:id/update" view=DietUpdatePage/>
            <Route path="/:date/:meal/:id/delete" view=DietDeletePage/>
        </Route>
    }
}

#[component]
pub fn DietLayout() -> impl IntoView {
    view! { <Outlet/> }
}
