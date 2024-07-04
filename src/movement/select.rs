use leptos::*;

use uuid::Uuid;

use crate::component::select::SelectUuidName;
use crate::component::template::{OptionError, OptionLoading};

pub type MoveSelectResource = Resource<(), Result<Vec<SelectUuidName>, ServerFnError>>;

#[server]
pub async fn get_movement_select() -> Result<Vec<SelectUuidName>, ServerFnError> {
    crate::auth::service::get_request_user()?;
    use crate::movement::model::Movement;
    let pool = expect_context::<sqlx::PgPool>();
    Ok(Movement::option_list_id(&pool).await?)
}

#[component]
pub fn MovementSelect(name: &'static str, #[prop(optional, into)] selected: Uuid) -> impl IntoView {
    let resource: MoveSelectResource = expect_context::<MoveSelectResource>();

    let response = move || {
        resource.and_then(|data| {
            data.iter()
                .map(|option| {
                    let value = option.id.to_string();
                    let name = option.name.clone();
                    let is_selected = option.id == selected;
                    view! {
                        <option prop:value=value prop:selected=move || is_selected>
                            {name}
                        </option>
                    }
                })
                .collect_view()
        })
    };
    view! {
        <label class="block">
            <span class="block mb-1 text-sm font-bold">"Exercise"</span>
            <select
                name=name
                class="block py-2 px-3 w-full bg-white rounded border focus:border-blue-500 focus:ring-2 focus:ring-blue-500 focus:outline-none"
            >
                <Transition fallback=OptionLoading>
                    <ErrorBoundary fallback=|_| {
                        view! { <OptionError/> }
                    }>{response}</ErrorBoundary>
                </Transition>
            </select>
        </label>
    }
}
