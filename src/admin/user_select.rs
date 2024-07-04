use leptos::*;

use uuid::Uuid;

use crate::component::select::SelectUuidName;
use crate::component::template::{OptionError, OptionLoading};
use crate::util::text::capitalize_and_replace;

#[server]
pub async fn get_user_select() -> Result<Vec<SelectUuidName>, ServerFnError> {
    crate::auth::service::extract_superuser_from_request()?;
    let pool = crate::setup::get_pool()?;
    Ok(crate::auth::model::User::option_list_id(&pool).await?)
}

pub type UserSelectResource = Resource<(), Result<Vec<SelectUuidName>, ServerFnError>>;

#[component]
pub fn UserSelect(
    name: &'static str,
    #[prop(default = name)] label: &'static str,
    #[prop(optional)] selected: Uuid,
) -> impl IntoView {
    let resource: UserSelectResource = expect_context::<UserSelectResource>();
    let response = move || {
        resource.and_then(|data| {
            data.clone()
                .into_iter()
                .map(|option| {
                    view! {
                        <option
                            prop:value=option.id.to_string()
                            prop:selected=move || option.id == selected
                        >
                            {option.name}
                        </option>
                    }
                })
                .collect_view()
        })
    };

    view! {
        <label class="block mb-4">
            <span class="block mb-1 font-bold">{capitalize_and_replace(label)}</span>
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
