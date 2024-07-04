use leptos::*;
use leptos_router::*;

use crate::movement::create_page::MovementCreatePage;
use crate::movement::delete_page::MovementDeletePage;
use crate::movement::detail_page::MovementDetailPage;
use crate::movement::list_page::MovementListPage;
use crate::movement::update_page::MovementUpdatePage;

use crate::muscle_group::create_page::MuscleGroupCreatePage;
use crate::muscle_group::delete_page::MuscleGroupDeletePage;
use crate::muscle_group::detail_page::MuscleGroupDetailPage;
use crate::muscle_group::list_page::MuscleGroupListPage;
use crate::muscle_group::update_page::MuscleGroupUpdatePage;

#[component(transparent)]
pub fn MovementRouter() -> impl IntoView {
    view! {
        <Route path="/exercises" view=MovementLayout>
            <Route path="/create" view=MovementCreatePage/>
            <Route path="/:slug" view=MovementDetailPage/>
            <Route path="/:slug/update" view=MovementUpdatePage/>
            <Route path="/:slug/delete" view=MovementDeletePage/>

            <Route path="/muscle-groups" view=MuscleGroupListPage/>
            <Route path="/muscle-groups/create" view=MuscleGroupCreatePage/>
            <Route path="/muscle-groups/:slug" view=MuscleGroupDetailPage/>
            <Route path="/muscle-groups/:slug/update" view=MuscleGroupUpdatePage/>
            <Route path="/muscle-groups/:slug/delete" view=MuscleGroupDeletePage/>

            <Route path="/" view=MovementListPage/>
        </Route>
    }
}

#[component]
pub fn MovementLayout() -> impl IntoView {
    view! { <Outlet/> }
}
