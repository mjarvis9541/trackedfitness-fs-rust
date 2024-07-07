use leptos::*;
use leptos_router::*;

use crate::auth::context::RequestUserContext;
use crate::component::template::{ErrorComponent, LoadingComponent};
use crate::util::param::{get_date, get_username};

use super::detail_page::get_profile_detail_latest;
use super::detail_table::ProfileDetailTable;

#[component]
pub fn ProfileSetupComponent() -> impl IntoView {
    view! {
        <p class="mb-4 text-gray-500">"You do not currently have a fitness profile set up."</p>
        <p class="mb-4 text-gray-500">
            "Use this feature to establish your body mass index (BMI), basal matabolic rate (BMR), total daily energy expenditure (TDEE) and to enable auto-generation of diet targets based on your current condition and fitness goals."
        </p>
        <div class="flex gap-2 justify-end pt-4">
            <A
                id="profile-create"
                class="block py-1.5 px-3 bg-amber-200 rounded hover:bg-gray-300"
                href="profile/create"
            >
                "Set up profile"
            </A>
        </div>
    }
}

#[component]
pub fn ProfileDetailPanel() -> impl IntoView {
    let params = use_params_map();
    let username = move || get_username(&params);
    let date = move || get_date(&params);
    let user_context = expect_context::<RequestUserContext>();
    let is_not_self = move || user_context.is_not_object_owner(username());
    let profile_resource = Resource::new(
        move || (username(), date()),
        |(username, date)| get_profile_detail_latest(username, date),
    );
    let profile_response = move || {
        profile_resource.and_then(|data| {
            data.as_ref().map_or_else(
                || {
                    view! {
                        <div>"No profile set up."</div>
                        <div class=("hidden", is_not_self)>
                            <ProfileSetupComponent/>
                        </div>
                    }
                },
                |data| {
                    let detail = format!("/users/{}/profile", data.username);
                    let update = format!("/users/{}/profile/update", data.username);
                    let delete = format!("/users/{}/profile/delete", data.username);
                    view! {
                        <ProfileDetailTable data=data.clone()/>
                        <div class=("hidden", is_not_self)>
                            <div class="flex gap-2 justify-end pt-4">
                                <a
                                    id="profile-detail"
                                    class="block py-1.5 px-3 bg-gray-200 rounded hover:bg-gray-300"
                                    href=detail
                                >
                                    "View"
                                </a>
                                <a
                                    id="profile-update"
                                    class="block py-1.5 px-3 bg-gray-200 rounded hover:bg-gray-300"
                                    href=update
                                >
                                    "Edit"
                                </a>
                                <a
                                    id="profile-delete"
                                    class="block py-1.5 px-3 bg-gray-200 rounded hover:bg-gray-300"
                                    href=delete
                                >
                                    "Delete"
                                </a>
                            </div>
                        </div>
                    }
                },
            )
        })
    };
    view! {
        <div class="p-4 bg-white border">
            <h2 class="mb-2 text-xl font-bold">"Profile"</h2>
            <Transition fallback=LoadingComponent>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors/> }
                }>{profile_response}</ErrorBoundary>
            </Transition>
        </div>
    }
}
