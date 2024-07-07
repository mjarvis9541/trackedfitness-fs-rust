use leptos::*;
use leptos_router::*;

use crate::auth::context::RequestUserContext;
use crate::component::icon::{IconClose, IconMenu};
use crate::component::link::{CircularIconLink, Link, LinkVariant, NotificationLink};
use crate::component::template::Backdrop;

#[component]
pub fn Navbar(username: String) -> impl IntoView {
    let initial = username.chars().next().unwrap();

    let top_nav = vec![
        (format!("/users/{}/diet", username), "Diet"),
        (format!("/users/{}/workouts", username), "Workouts"),
        (format!("/users/{}/week", username), "Week"),
        (format!("/users/{}/month", username), "Month"),
        ("/food".to_string(), "Food"),
        ("/exercises".to_string(), "Exercises"),
    ];
    let side_nav = vec![
        (format!("/users/{}", username), "Profile"),
        (format!("/users/{}/diet", username), "Diet"),
        (format!("/users/{}/workouts", username), "Workouts"),
        (format!("/users/{}/progress", username), "Progress"),
        (format!("/users/{}/diet-targets", username), "Diet Targets"),
        ("meal-of-day".to_string(), "Meal of Day"),
        (format!("/users/{}/week", username), "Week"),
        (format!("/users/{}/month", username), "Month"),
        ("/food".to_string(), "Food"),
        ("/food/brands".to_string(), "Brands"),
        ("/food/meals".to_string(), "Meals"),
        ("/exercises".to_string(), "Exercises"),
        ("/exercises/muscle-groups".to_string(), "Muscle Groups"),
    ];
    let side_nav_lower = vec![
        ("/settings".to_string(), "Settings"),
        ("/help".to_string(), "Help"),
        ("/privacy-policy".to_string(), "Privacy Policy"),
        ("/terms-of-service".to_string(), "Terms of Service"),
        ("/logout".to_string(), "Log out"),
    ];

    let user_href = format!("/users/{}", username);

    let show_menu = RwSignal::new(false);
    let close_menu = move |_| show_menu.update(|value| *value = false);
    let toggle_menu = move |_| show_menu.update(|value| *value = !*value);
    let is_hidden = move || show_menu.with(|value| !*value);

    let top_nav_view = top_nav.into_iter().map(|(href, text)| view! { <Link variant=LinkVariant::Navigation href=href text=text exact=true/> }).collect_view();
    let side_nav_view = side_nav.into_iter().map(|(href, text)| view! { <Link variant=LinkVariant::Navigation href=href text=text exact=true/> }).collect_view();
    let side_nav_view_lower = side_nav_lower.into_iter().map(|(href, text)| view! { <Link variant=LinkVariant::Navigation href=href text=text exact=true/> }).collect_view();

    let request_user = expect_context::<RequestUserContext>();

    view! {
        <Backdrop show_menu/>

        <nav class="flex overflow-hidden sticky top-0 z-10 justify-between p-2 bg-zinc-900 text-zinc-100">
            <section class="flex bg-zinc-800">
                <button class="p-2 hover:bg-zinc-600" on:click=toggle_menu>
                    <IconMenu/>
                </button>
                <A class="flex items-center px-4 font-bold hover:bg-zinc-600" href="/">
                    "Trackedfitness"
                </A>
            </section>
            <section class="flex gap-x-2 items-center">
                <Show when=move || request_user.is_superuser()>
                    <div>
                        <Link
                            variant=LinkVariant::Navigation
                            href="/admin/users"
                            text="Admin"
                            exact=true
                        />
                    </div>
                </Show>
                <div class="hidden md:flex">{top_nav_view}</div>
                <div class="flex gap-x-2 items-center">
                    <NotificationLink href="/settings/followers"/>
                    <CircularIconLink href=user_href initial/>
                </div>
            </section>
        </nav>
        <nav
            class="fixed top-0 z-30 flex h-full w-72 transform flex-col overflow-hidden bg-zinc-900 pb-2 text-zinc-100 transition-transform duration-300"
            class=("-translate-x-full", is_hidden)
        >
            <section class="flex p-2" on:click=close_menu>
                <A class="flex-1 p-2 px-4 font-bold hover:bg-zinc-600" href="/">
                    "Trackedfitness"
                </A>
                <button class="p-2 hover:bg-zinc-600">
                    <IconClose/>
                </button>
            </section>
            <section class="flex-1 space-y-2 overflow-y-auto p-2" on:click=close_menu>
                {side_nav_view}
            </section>
            <section class="space-y-2 p-2" on:click=close_menu>
                {side_nav_view_lower}
            </section>
        </nav>
    }
}

#[component]
pub fn UnauthNavbar() -> impl IntoView {
    view! {
        <nav class="flex overflow-hidden sticky top-0 z-10 justify-between p-2 bg-zinc-900 text-zinc-100">
            <section class="flex">
                <A class="flex items-center p-2 font-bold hover:bg-zinc-600" href="/">
                    "Trackedfitness"
                </A>
            </section>
            <section class="flex">
                <Link variant=LinkVariant::Navigation text="Log in" href="login"/>
                <Link variant=LinkVariant::Navigation text="Sign up" href="signup"/>
            </section>
        </nav>
    }
}
