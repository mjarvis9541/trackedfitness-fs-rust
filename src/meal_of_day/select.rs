use leptos::*;

use uuid::Uuid;

use crate::component::select::SelectUuidName;
use crate::component::template::{OptionError, OptionLoading};

#[server]
pub async fn get_meal_of_day_select() -> Result<Vec<SelectUuidName>, ServerFnError> {
    use crate::auth::service::get_request_user;
    get_request_user()?;
    let pool = expect_context::<sqlx::PgPool>();
    Ok(crate::meal_of_day::model::MealOfDay::option_list_id(&pool).await?)
}

#[component]
pub fn MealOfDaySelect(#[prop(optional)] selected: Uuid) -> impl IntoView {
    let resource = Resource::once(get_meal_of_day_select);

    let response = move || {
        resource.and_then(|data| {
            data.clone()
                .into_iter()
                .map(|option| {
                    view! {
                        <option value=option.id.to_string() selected=move || option.id == selected>
                            {option.name}
                        </option>
                    }
                })
                .collect_view()
        })
    };
    view! {
        <label class="block mb-4">
            <span class="block mb-1 font-bold">"Meal of Day"</span>
            <select
                name="meal_of_day_id"
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
