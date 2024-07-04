use leptos::*;
use leptos_router::*;

use crate::auth::context::RequestUserContext;
use crate::component::template::{CreateButton, ErrorComponent, LoadingComponent};
use crate::util::param::{get_date, get_username};

use super::detail_page::get_progress_detail_latest;
use super::detail_table::ProgressDetailTable;

#[component]
pub fn ProgressDetailPanelComponent() -> impl IntoView {
    let params = use_params_map();
    let username = move || get_username(&params);
    let date = move || get_date(&params);
    let user_context = expect_context::<RequestUserContext>();

    let is_not_self = move || user_context.is_not_object_owner(username());
    let resource = Resource::new(
        move || (username(), date()),
        |(username, date)| get_progress_detail_latest(username, date),
    );
    let response = move || {
        resource.and_then(|data| {
            data.as_ref().map_or_else(
                || {
                    let create_href =
                        format!("/users/{}/progress/create?date={}", username(), date());
                    view! {
                        <div>"No progress logged today or in the past."</div>
                        <div class=("hidden", is_not_self)>
                            <CreateButton text="Log Progress" create_href/>
                        </div>
                    }
                },
                |data| {
                    let is_current_date = data.date == date();
                    let detail = format!("/users/{}/progress/{}", data.username, data.date);
                    let update = format!("/users/{}/progress/{}/update", data.username, data.date);
                    let delete = format!("/users/{}/progress/{}/delete", data.username, data.date);
                    let create = format!("/users/{}/progress/create?date={}", username(), date());
                    view! {
                        <ProgressDetailTable data=data.clone()/>
                        <div class=("hidden", is_not_self)>
                            <div class="flex gap-2 justify-end pt-4">
                                <a
                                    id="progress-detail"
                                    class="block py-1.5 px-3 bg-gray-200 rounded hover:bg-gray-300"
                                    href=detail
                                >
                                    "View"
                                </a>
                                <a
                                    id="progress-update"
                                    class="block py-1.5 px-3 bg-gray-200 rounded hover:bg-gray-300"
                                    href=update
                                >
                                    "Edit"
                                </a>
                                <a
                                    id="progress-delete"
                                    class="block py-1.5 px-3 bg-gray-200 rounded hover:bg-gray-300"
                                    href=delete
                                >
                                    "Delete"
                                </a>
                            </div>
                            <div class=("hidden", is_current_date)>
                                <CreateButton text="Log Progress" create_href=create/>
                            </div>
                        </div>
                    }
                },
            )
        })
    };
    view! {
        <div class="p-4 bg-white border">
            <h2 class="mb-2 text-xl font-bold">"Progress"</h2>
            <Transition fallback=LoadingComponent>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors/> }
                }>{response}</ErrorBoundary>
            </Transition>
        </div>
    }
}
