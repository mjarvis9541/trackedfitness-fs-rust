use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::admin::router::AdminRouter;
use crate::auth::context::RequestUserContext;
use crate::auth::login_page::{Login, LoginPage};
use crate::auth::logout_page::{Logout, LogoutPage};
use crate::auth::model::RequestUser;
use crate::auth::password_reset_complete_page::PasswordResetCompletePage;
use crate::auth::password_reset_confirm_page::PasswordResetConfirmPage;
use crate::auth::password_reset_done_page::PasswordResetRequestDonePage;
use crate::auth::password_reset_page::PasswordResetRequestPage;
use crate::auth::protected_route::ProtectedRoute;
use crate::auth::signup_complete_page::SignupCompletePage;
use crate::auth::signup_confirm_page::{SignupConfirm, SignupConfirmPage};
use crate::auth::signup_done_page::SignupDonePage;
use crate::auth::signup_page::SignupPage;
use crate::auth::signup_resend_page::SignupResendPage;
use crate::component::navbar::{Navbar, UnauthNavbar};
use crate::component::template::NotFound;
// use crate::follower::component::pending_follower_request_notification;
use crate::food::router::FoodRouter;
use crate::meal_of_day::create_page::MealOfDayCreatePage;
use crate::meal_of_day::delete_page::MealOfDayDeletePage;
use crate::meal_of_day::detail_page::MealOfDayDetailPage;
use crate::meal_of_day::list_page::MealOfDayListPage;
use crate::meal_of_day::update_page::MealOfDayUpdatePage;
use crate::movement::router::MovementRouter;
// use crate::training_plan::router::TrainingPlanRouter;
use crate::user::router::UserRouter;
use crate::user_setting::settings_router::UserSettingsRouter;
// use crate::util::use_interval::use_interval;
use crate::web::help_page::HelpPage;
use crate::web::landing_page::LandingPage;
use crate::web::privacy_policy_page::PrivacyPage;
use crate::web::terms_of_service_page::TermsPage;

pub type UserResource = Resource<(usize, usize, usize), Result<RequestUser, ServerFnError>>;

#[cfg(feature = "ssr")]
use crate::auth::service::extract_user_from_request;

#[server(endpoint = "authenticate")]
pub async fn authenticate() -> Result<RequestUser, ServerFnError> {
    let user = extract_user_from_request()?.unwrap_or_default();
    Ok(user)
}

#[derive(Debug, Copy, Clone)]
pub struct FollowerCount(pub RwSignal<i64>);

impl FollowerCount {
    pub fn new() -> Self {
        Self(RwSignal::new(0))
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let request_user_ctx = RequestUserContext::new();
    provide_context(request_user_ctx);
    let follower_count = FollowerCount::new();
    provide_context(follower_count);
    // let follower_req = create_action(|&()| pending_follower_request_notification());
    // use_interval(60000, move || {
    //     leptos::logging::log!("Checking pending follower requests.");
    //     follower_req.dispatch(())
    // });
    // let value = move || {
    //     follower_req.value().get().map(|v| match v {
    //         Ok(count) => follower_count.0.update(|v| *v = count),
    //         Err(_) => follower_count.0.set(0),
    //     })
    // };
    let login = Action::<Login, _>::server();
    let logout = Action::<Logout, _>::server();
    let signup_confirm = Action::<SignupConfirm, _>::server();
    let resource: UserResource = Resource::new(
        move || {
            (
                login.version().get(),
                logout.version().get(),
                signup_confirm.version().get(),
            )
        },
        move |_| authenticate(),
    );
    provide_context(resource);
    let response = move || {
        resource.and_then(|user| {
            if user.is_active {
                request_user_ctx.user.update(|v| v.clone_from(user));
                let username = user.username.clone();
                view! { <Navbar username/> }
            } else {
                view! { <UnauthNavbar/> }
            }
        })
    };

    view! {
        <Title text="Welcome" formatter=|text| format!("{text} - Trackedfitness")/>
        <Stylesheet id="leptos" href="/pkg/trackedfitness.css"/>
        <Meta
            name="description"
            content="Track your diet and workout progress effortlessly with our calorie-counting and training tracking web app. Achieve your fitness goals efficiently by monitoring your intake and activities in one convenient platform. Start optimizing your health journey today!"
        />
        <Body class="text-sm text-gray-900 bg-gray-100"/>
        <Router fallback=|| NotFound.into_view()>
            <Transition>
                <ErrorBoundary fallback=|_| {
                    view! { <UnauthNavbar/> }
                }>{response}</ErrorBoundary>
            </Transition>
            // <div class="hidden">{value}</div>
            <Routes>
                <AdminRouter/>
                <Route path="/" view=LandingPage/>
                <Route path="/privacy-policy" view=PrivacyPage/>
                <Route path="/terms-of-service" view=TermsPage/>
                <Route path="/help" view=HelpPage/>
                <Route path="/login" view=move || view! { <LoginPage login/> }/>
                <Route path="/logout" view=move || view! { <LogoutPage logout/> }/>
                <Route path="/signup" view=SignupPage/>
                <Route path="/signup/email-sent" view=SignupDonePage/>
                <Route path="/signup/resend-email" view=SignupResendPage/>
                <Route
                    path="/signup/confirm"
                    view=move || view! { <SignupConfirmPage action=signup_confirm/> }
                />
                <Route path="/signup/complete" view=SignupCompletePage/>
                <Route path="/password-reset" view=PasswordResetRequestPage/>
                <Route path="/password-reset/email-sent" view=PasswordResetRequestDonePage/>
                <Route path="/password-reset/confirm" view=PasswordResetConfirmPage/>
                <Route path="/password-reset/complete" view=PasswordResetCompletePage/>

                <Route path="" view=ProtectedRoute>
                    <FoodRouter/>
                    <Route path="/meal-of-day" view=MealOfDayListPage/>
                    <Route path="/meal-of-day/create" view=MealOfDayCreatePage/>
                    <Route path="/meal-of-day/:slug" view=MealOfDayDetailPage/>
                    <Route path="/meal-of-day/:slug/update" view=MealOfDayUpdatePage/>
                    <Route path="/meal-of-day/:slug/delete" view=MealOfDayDeletePage/>
                    <UserRouter/>
                    <MovementRouter/>
                    <UserSettingsRouter/>
                </Route>
            </Routes>
        </Router>
    }
}
