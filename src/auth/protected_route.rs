use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::app::UserResource;

#[component]
pub fn Unauthorized() -> impl IntoView {
    view! {
        <main class="p-8">
            <div class="p-4 mx-auto max-w-md bg-white border shadow-md">
                <h1 class="mb-4 text-xl font-bold">"Unauthorized"</h1>
                <p class="mb-4">"You need to be logged in to view this page."</p>
                <a class="text-blue-500 hover:underline" href="/login">
                    "Log in"
                </a>
            </div>
        </main>
    }
}

#[component]
pub fn ProtectedRoute() -> impl IntoView {
    let resource = expect_context::<UserResource>();
    let can_view = move || resource.with(|res| matches!(res, Some(Ok(user)) if user.is_active));
    view! {
        <Title text="Loading..."/>
        <LoginRequired show=can_view fallback=|| view! { <Unauthorized/> }>
            <Outlet/>
        </LoginRequired>
    }
}

#[component]
pub fn AdminProtectedRoute() -> impl IntoView {
    let resource = expect_context::<UserResource>();
    let can_view = move || resource.with(|res| matches!(res, Some(Ok(user)) if user.is_superuser));
    view! {
        <Title text="Loading..."/>
        <LoginRequired show=can_view fallback=|| view! { <Unauthorized/> }>
            <Outlet/>
        </LoginRequired>
    }
}

#[component]
pub fn LoginRequired<S, F, IV>(show: S, fallback: F, children: ChildrenFn) -> impl IntoView
where
    S: Fn() -> bool + Copy + 'static,
    F: Fn() -> IV + 'static,
    IV: IntoView,
{
    let fallback = store_value(fallback);
    let children = store_value(children);
    view! {
        <Title text="Loading..."/>
        <Suspense>
            <Show when=show fallback=move || fallback.with_value(|fallback| fallback())>
                {children.with_value(|children| children())}
            </Show>
        </Suspense>
    }
}

// <LoginRequired fallback=Unauthorized show=is_authenticated>
//     <Outlet/>
// </LoginRequired>
// <Suspense>
// <Show when=can_view fallback=|| view! { <Unauthorized/> }>
//     <Outlet/>
// </Show>
// </Suspense>
