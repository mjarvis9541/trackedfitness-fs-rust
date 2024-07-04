use leptos::*;
use leptos_router::*;

use super::admin_follower_list_page::AdminFollowerListPage;
use super::detail_page::AdminDetailPage;
use super::follower_detail_page::AdminFollowerDetailPage;
use super::layout::AdminLayout;
use super::user_block_detail_page::AdminUserBlockDetailPage;
use super::user_block_list_page::AdminUserBlockListPage;
use super::user_create_page::AdminUserCreatePage;
use super::user_detail_page::AdminUserDetailPage;
use super::user_list_page::AdminUserListPage;
use super::user_stats_list::AdminUserStatListPage;
use crate::auth::protected_route::AdminProtectedRoute;

#[component(transparent)]
pub fn AdminRouter() -> impl IntoView {
    view! {
        <Route path="/admin" view=AdminProtectedRoute>
            <Route path="" view=AdminLayout>
                <Route path="/users" view=AdminUserListPage/>
                <Route path="/users/create" view=AdminUserCreatePage/>
                <Route path="/users/:id" view=AdminUserDetailPage/>
                <Route path="/followers" view=AdminFollowerListPage/>
                <Route path="/followers/:id" view=AdminFollowerDetailPage/>
                <Route path="/blocked-users" view=AdminUserBlockListPage/>
                <Route path="/blocked-users/:id" view=AdminUserBlockDetailPage/>
                <Route path="/user-stats" view=AdminUserStatListPage/>
                <Route path="/" view=AdminDetailPage/>
            </Route>
        </Route>
    }
}
