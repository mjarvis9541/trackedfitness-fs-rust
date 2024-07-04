use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::component::icon::{IconHome, IconUsers};
use crate::component::link::{Link, LinkVariant};

#[component]
pub fn AdminLayout() -> impl IntoView {
    view! {
        <Title text="Admin"/>
        <main class="md:p-4">
            <div class="grid grid-cols-4 lg:grid-cols-12">
                <section class="flex overflow-auto col-span-4 space-y-2 h-full whitespace-nowrap lg:flex-col lg:col-span-2">
                    <Link exact=true variant=LinkVariant::UserNavLink text="Admin" href="/admin">
                        <IconHome/>
                    </Link>
                    <Link
                        exact=true
                        variant=LinkVariant::UserNavLink
                        text="Users"
                        href="/admin/users"
                    >
                        <IconUsers/>
                    </Link>
                    <Link
                        exact=true
                        variant=LinkVariant::UserNavLink
                        text="User Stats"
                        href="/admin/user-stats"
                    >
                        <IconUsers/>
                    </Link>
                    <Link
                        exact=true
                        variant=LinkVariant::UserNavLink
                        text="User Followers"
                        href="/admin/followers"
                    >
                        <IconUsers/>
                    </Link>
                    <Link
                        exact=true
                        variant=LinkVariant::UserNavLink
                        text="Blocked Users"
                        href="/admin/blocked-users"
                    >
                        <IconUsers/>
                    </Link>
                    <div class="flex-grow"></div>
                </section>
                <section class="col-span-4 lg:col-span-10">
                    <Outlet/>
                </section>
            </div>
        </main>
    }
}
