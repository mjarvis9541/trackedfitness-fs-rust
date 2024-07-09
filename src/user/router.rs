use leptos::*;
use leptos_router::*;

use super::layout::{UserLayout, UserLayoutResource};
use crate::diet::router::DietRouter;
use crate::follower::follower_list_page::UserFollowerListPage;
use crate::follower::following_list_page::UserFollowingPage;
use crate::profile::create_page::ProfileCreatePage;
use crate::profile::delete_page::ProfileDeletePage;
use crate::profile::detail_page::ProfileDetailPage;
use crate::profile::update_page::ProfileUpdatePage;

use crate::user::detail_page::UserDetailPage;
use crate::workout::router::WorkoutRouter;

use crate::summary::month_page::UserSummaryMonthPage;
use crate::summary::week_page::UserSummaryWeekPage;

use crate::diet_target::create_page::DietTargetCreatePage;
use crate::diet_target::delete_page::DietTargetDeletePage;
use crate::diet_target::detail_page::DietTargetDetailPage;
use crate::diet_target::list_page::DietTargetListPage;
use crate::diet_target::update_page::DietTargetUpdatePage;

use crate::progress::create_page::ProgressCreatePage;
use crate::progress::delete_page::ProgressDeletePage;
use crate::progress::detail_page::ProgressDetailPage;
use crate::progress::list_page::ProgressListPage;
use crate::progress::update_page::ProgressUpdatePage;

#[component(transparent)]
pub fn UserRouter() -> impl IntoView {
    view! {
        <Route path="/users/:username" view=UserLayout>
            <Route path="/followers" view=UserFollowerListPage/>
            <Route path="/following" view=UserFollowingPage/>

            <Route path="" view=UserFollowerProtectedRoute>
                <Route path="/:date?" view=UserDetailPage/>
                <Route path="/diet-targets" view=DietTargetListPage/>
                <Route path="/diet-targets/create" view=DietTargetCreatePage/>
                <Route path="/diet-targets/:date" view=DietTargetDetailPage/>
                <Route path="/diet-targets/:date/update" view=DietTargetUpdatePage/>
                <Route path="/diet-targets/:date/delete" view=DietTargetDeletePage/>

                <Route path="/progress" view=ProgressListPage/>
                <Route path="/progress/create" view=ProgressCreatePage/>
                <Route path="/progress/:date" view=ProgressDetailPage/>
                <Route path="/progress/:date/update" view=ProgressUpdatePage/>
                <Route path="/progress/:date/delete" view=ProgressDeletePage/>

                <Route path="/week/:date?" view=UserSummaryWeekPage/>
                <Route path="/month/:date?" view=UserSummaryMonthPage/>

                <Route path="/profile" view=ProfileDetailPage/>
                <Route path="/profile/create" view=ProfileCreatePage/>
                <Route path="/profile/update" view=ProfileUpdatePage/>
                <Route path="/profile/delete" view=ProfileDeletePage/>

                <DietRouter/>
                <WorkoutRouter/>
            </Route>
        </Route>
    }
}

#[component]
pub fn UserFollowerProtectedRoute() -> impl IntoView {
    let resource = expect_context::<UserLayoutResource>();
    let can_view = move || resource.with(|opt| matches!(opt, Some(Ok(user)) if user.can_view));

    view! {
        <Suspense>
            <Show when=can_view fallback=PrivateUserPage>
                <Outlet/>
            </Show>
        </Suspense>
    }
}

#[component]
pub fn PrivateUserPage() -> impl IntoView {
    view! {
        <div class="p-4">
            "You are not currently following this user and they have their profile set to private."
        </div>
    }
}
