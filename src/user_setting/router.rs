use leptos::*;
use leptos_router::*;

use crate::auth::email_change_complete_page::EmailChangeCompletePage;
use crate::auth::email_change_confirm_page::EmailChangeConfirmPage;
use crate::auth::email_change_done_page::EmailChangeRequestDonePage;
use crate::auth::email_change_page::EmailChangeRequestPage;
use crate::profile::upload_page::ProfileImageUploadPage;
use crate::user_setting::account_page::UserAccountSettingsPage;
use crate::user_setting::follower_page::FollowerRequestListPage;
use crate::user_setting::layout::UserSettingsLayout;
use crate::user_setting::password_change_page::PasswordUpdatePage;
use crate::user_setting::site_statistics_page::UserStatsDetailPage;
use crate::user_setting::user_block_page::UserBlockListPage;

#[component(transparent)]
pub fn UserSettingsRouter() -> impl IntoView {
    view! {
        <Route path="/settings" view=UserSettingsLayout>
            <Route path="/change-password" view=PasswordUpdatePage/>
            <Route path="/stats" view=UserStatsDetailPage/>
            <Route path="/followers" view=FollowerRequestListPage/>
            <Route path="/blocked-users" view=UserBlockListPage/>
            <Route path="/change-email" view=EmailChangeRequestPage/>
            <Route path="/change-email/email-sent" view=EmailChangeRequestDonePage/>
            <Route path="/change-email/confirm" view=EmailChangeConfirmPage/>
            <Route path="/change-email/complete" view=EmailChangeCompletePage/>
            <Route path="/upload" view=ProfileImageUploadPage/>
            <Route path="/" view=UserAccountSettingsPage/>
        </Route>
    }
}
