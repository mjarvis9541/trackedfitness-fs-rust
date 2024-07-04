use leptos::*;
use leptos_meta::*;

use crate::component::date_navigation::DateNavigation;
use crate::diet_target::component::DietTargetDetailPanelComponent;
use crate::profile::component::ProfileDetailPanelComponent;
use crate::progress::component::ProgressDetailPanelComponent;

#[component]
pub fn UserDetailPage() -> impl IntoView {
    view! {
        <Title text="User Detail"/>
        <div class="lg:px-4">

            <DateNavigation/>

            <div class="grid grid-cols-4 gap-4 md:grid-cols-8 lg:grid-cols-12">
                <section class="col-span-4">
                    <ProfileDetailPanelComponent/>
                </section>

                <section class="col-span-4">
                    <ProgressDetailPanelComponent/>
                </section>

                <section class="col-span-4">
                    <DietTargetDetailPanelComponent/>
                </section>
            </div>

        </div>
    }
}
