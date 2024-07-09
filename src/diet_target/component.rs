use leptos::*;
use leptos_router::*;

use crate::auth::context::RequestUserContext;
use crate::component::template::{CreateButton, ErrorComponent, LoadingComponent};
use crate::util::param::{get_date, get_username};

use super::detail_page::get_diet_target_detail_latest;
use super::detail_table::DietTargetDetailTable;
use super::from_profile_form::{TargetFromProfileCreate, TargetFromProfileCreateForm};

#[component]
pub fn DietTargetDetailPanelComponent() -> impl IntoView {
    let params = use_params_map();
    let username = move || get_username(&params);
    let date = move || get_date(&params);

    let user_context = expect_context::<RequestUserContext>();
    let is_self = move || user_context.is_object_owner(username());
    let is_not_self = move || user_context.is_not_object_owner(username());

    let diet_target_create = Action::<TargetFromProfileCreate, _>::server();

    let diet_target_resource = Resource::new(
        move || (username(), date(), diet_target_create.version().get()),
        |(username, date, _)| get_diet_target_detail_latest(username, date),
    );
    let diet_target_response = move || {
        diet_target_resource.and_then(|data| {
            data.as_ref().map_or_else(
                || {
                    let create =
                        format!("/users/{}/diet-targets/create?date={}", username(), date());
                    view! {
                        <div class="mb-4 text-gray-500" class=("hidden", is_self)>
                            "This user has not set up a diet target."
                        </div>
                        <div class=("hidden", is_not_self)>
                            <p class="mb-4 text-gray-500">
                                "You do not currently have a diet target set up."
                            </p>
                            <p class="mb-4 text-gray-500">
                                "Use this feature to set daily calorie and macronutrient targets."
                            </p>
                            <CreateButton
                                text="Set up Diet Target"
                                create_href=create
                                attr:id="diet-target-create"
                            />
                        </div>
                    }
                },
                |data| {
                    let is_current_date = data.date == date();
                    let detail = format!("/users/{}/diet-targets/{}", data.username, data.date);
                    let update =
                        format!("/users/{}/diet-targets/{}/update", data.username, data.date);
                    let delete =
                        format!("/users/{}/diet-targets/{}/delete", data.username, data.date);
                    let create =
                        format!("/users/{}/diet-targets/create?date={}", username(), date());
                    view! {
                        <DietTargetDetailTable data=data.clone()/>

                        <div class=("hidden", is_not_self)>
                            <div class="flex gap-2 justify-end pt-4">
                                <a
                                    id="diet-target-detail"
                                    class="block py-1.5 px-3 bg-gray-200 rounded hover:bg-gray-300"
                                    href=detail
                                >
                                    "View"
                                </a>
                                <a
                                    id="diet-target-update"
                                    class="block py-1.5 px-3 bg-gray-200 rounded hover:bg-gray-300"
                                    href=update
                                >
                                    "Edit"
                                </a>
                                <a
                                    id="diet-target-delete"
                                    class="block py-1.5 px-3 bg-gray-200 rounded hover:bg-gray-300"
                                    href=delete
                                >
                                    "Delete"
                                </a>
                            </div>

                            <div class=("hidden", is_current_date)>
                                <CreateButton
                                    text="New"
                                    create_href=create
                                    attr:id="diet-target-create"
                                />
                                <TargetFromProfileCreateForm action=diet_target_create/>
                            </div>
                        </div>
                    }
                },
            )
        })
    };
    view! {
        <div class="p-4 bg-white border">
            <h2 class="mb-2 text-xl font-bold">"Diet Target"</h2>
            <Transition fallback=LoadingComponent>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors/> }
                }>{diet_target_response}</ErrorBoundary>
            </Transition>
        </div>
    }
}
