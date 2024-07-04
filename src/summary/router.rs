use leptos::*;
use leptos_router::*;

use super::diet_target_week::DietTargetWeekSummaryComponent;

#[component(transparent)]
pub fn SummaryRouter() -> impl IntoView {
    view! {
        <Route path="" view=Layout>
            <Route path="/week" view=DietTargetWeekSummaryComponent/>
        </Route>
    }
}

#[component]
pub fn Layout() -> impl IntoView {
    view! { <Outlet/> }
}
