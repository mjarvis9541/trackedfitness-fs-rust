use leptos::*;
use leptos_router::*;

use uuid::Uuid;

use crate::brand::upload_page::BrandImageUploadPage;
use crate::component::link::{Link, LinkVariant};

use super::create_page::FoodCreatePage;
use super::delete_page::FoodDeletePage;
use super::detail_page::FoodDetailPage;
use super::list_page::FoodListPage;
use super::update_page::FoodUpdatePage;

use crate::brand::create_page::BrandCreatePage;
use crate::brand::delete_page::BrandDeletePage;
use crate::brand::detail_page::BrandDetailPage;
use crate::brand::list_page::BrandListPage;
use crate::brand::update_page::BrandUpdatePage;

use crate::meal::create_page::MealCreatePage;
use crate::meal::delete_page::MealDeletePage;
use crate::meal::detail_page::MealDetailPage;
use crate::meal::list_page::MealListPage;
use crate::meal::update_page::MealUpdatePage;

use crate::meal_food::delete_page::MealFoodDeletePage;
use crate::meal_food::detail_page::MealFoodDetailPage;
use crate::meal_food::update_page::MealFoodUpdatePage;

#[derive(Debug, PartialEq, Params, Clone)]
pub struct MealFoodParam {
    pub meal_food_id: Uuid,
}

#[component(transparent)]
pub fn FoodRouter() -> impl IntoView {
    view! {
        <Route path="/food" view=FoodLayout>
            <Route path="/create" view=FoodCreatePage/>
            <Route path="/:slug" view=FoodDetailPage/>
            <Route path="/:slug/update" view=FoodUpdatePage/>
            <Route path="/:slug/delete" view=FoodDeletePage/>

            <Route path="/brands" view=BrandListPage/>
            <Route path="/brands/create" view=BrandCreatePage/>
            <Route path="/brands/:slug" view=BrandDetailPage/>
            <Route path="/brands/:slug/update" view=BrandUpdatePage/>
            <Route path="/brands/:slug/upload" view=BrandImageUploadPage/>
            <Route path="/brands/:slug/delete" view=BrandDeletePage/>

            <Route path="/meals" view=MealListPage/>
            <Route path="/meals/create" view=MealCreatePage/>
            <Route path="/meals/:id" view=MealDetailPage/>
            <Route path="/meals/:id/update" view=MealUpdatePage/>
            <Route path="/meals/:id/delete" view=MealDeletePage/>

            <Route path="/meals/:id/:meal_food_id" view=MealFoodDetailPage/>
            <Route path="/meals/:id/:meal_food_id/update" view=MealFoodUpdatePage/>
            <Route path="/meals/:id/:meal_food_id/delete" view=MealFoodDeletePage/>

            <Route path="/" view=FoodListPage/>
        </Route>
    }
}

#[component]
pub fn FoodLayout() -> impl IntoView {
    view! {
        <nav class="flex bg-zinc-800 text-zinc-100">
            <Link variant=LinkVariant::Navigation text="Food" href="" exact=true/>
            <Link variant=LinkVariant::Navigation text="Brands" href="brands" exact=true/>
            <Link variant=LinkVariant::Navigation text="Meals" href="meals" exact=true/>
        </nav>
        <Outlet/>
    }
}
