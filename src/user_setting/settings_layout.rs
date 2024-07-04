use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::component::link::{Link, LinkVariant};

#[component]
pub fn UserSettingsLayout() -> impl IntoView {
    view! {
        <Title text="Settings"/>
        <main class="md:p-4">

            <div class="grid grid-cols-4 lg:grid-cols-12">
                <section class="flex overflow-auto col-span-4 h-full whitespace-nowrap lg:flex-col lg:col-span-2">
                    <Link exact=true variant=LinkVariant::UserNavLink href="" text="Account"/>
                    <Link
                        exact=true
                        variant=LinkVariant::UserNavLink
                        href="followers"
                        text="Followers"
                    />
                    <Link
                        exact=true
                        variant=LinkVariant::UserNavLink
                        href="blocked-users"
                        text="Blocked Users"
                    />
                    <Link
                        exact=true
                        variant=LinkVariant::UserNavLink
                        href="stats"
                        text="Site Stats"
                    />
                    <Link
                        exact=true
                        variant=LinkVariant::UserNavLink
                        href="change-email"
                        text="Change Email"
                    />
                    <Link
                        exact=true
                        variant=LinkVariant::UserNavLink
                        href="change-password"
                        text="Change Password"
                    />
                    <Link
                        exact=true
                        variant=LinkVariant::UserNavLink
                        href="upload"
                        text="Profile Picture"
                    />
                </section>
                <section class="col-span-4 lg:col-span-10">
                    <Outlet/>
                </section>
            </div>
        </main>
    }
}
