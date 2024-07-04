use leptos::*;
use leptos_meta::*;

use crate::component::date_navigation::DateNavigation;
use crate::summary::diet_target_week::DietTargetWeekSummaryComponent;
use crate::summary::diet_week::DietWeekSummaryComponent;
use crate::util::datetime::Resolution;

#[component]
pub fn UserSummaryWeekPage() -> impl IntoView {
    view! {
        <Title text="Week Summary"/>
        <main class="m-4 p-4 bg-white border">
            <nav>
                <DateNavigation resolution=Resolution::Week/>
            </nav>
            <div class="overflow-x-auto mb-4">
                <DietWeekSummaryComponent/>
            </div>
            <div class="overflow-x-auto">
                <DietTargetWeekSummaryComponent/>
            </div>
        </main>
    }
}
