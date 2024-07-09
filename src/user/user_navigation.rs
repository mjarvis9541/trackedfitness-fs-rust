use leptos::*;

use crate::component::icon::{IconCalendar, IconFile, IconSettings};
use crate::component::link::{Link, LinkVariant};

#[component]
pub fn UserNavigation() -> impl IntoView {
    view! {
        <div class="flex flex-wrap gap-2">
            <Link exact=true variant=LinkVariant::UserNavLink href="diet" text="Diet">
                <IconFile/>
            </Link>
            <Link exact=true variant=LinkVariant::UserNavLink href="workouts" text="Workouts">
                <IconFile/>
            </Link>
            <Link
                exact=true
                variant=LinkVariant::UserNavLink
                href="diet-targets"
                text="Diet Targets"
            >
                <IconFile/>
            </Link>
            <Link exact=true variant=LinkVariant::UserNavLink href="progress" text="Progress">
                <IconFile/>
            </Link>
            <Link exact=true variant=LinkVariant::UserNavLink href="week" text="Week">
                <IconCalendar/>
            </Link>
            <Link exact=true variant=LinkVariant::UserNavLink href="month" text="Month">
                <IconCalendar/>
            </Link>
            <Link exact=true variant=LinkVariant::UserNavLink href="/settings" text="Settings">
                <IconSettings/>
            </Link>
        </div>
    }
}
