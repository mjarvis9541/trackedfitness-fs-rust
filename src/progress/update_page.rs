use leptos::*;
use leptos_router::*;

use chrono::prelude::*;
use rust_decimal::Decimal;

use crate::component::button::SubmitButton;
use crate::component::input::{NumberInput, TextInputImproved};
use crate::component::template::{DetailPageTemplate, ErrorComponent, LoadingComponent};
use crate::util::param::{get_date, get_username};
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

use super::detail_page::get_progress_detail;

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user,
    error::Error,
    progress::model::{Progress, ProgressBase},
    setup::get_pool,
};

#[server]
pub async fn progress_update(
    username: String,
    date: NaiveDate,
    weight_kg: Option<Decimal>,
    energy_burnt: Option<i32>,
    notes: Option<String>,
    redirect_to: Option<String>,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let object = ProgressBase::get_by_username_date(&pool, &username, date)
        .await?
        .ok_or(Error::NotFound)?;
    object.can_update(&user).await?;

    Progress::validate(date, weight_kg, energy_burnt, notes.clone())?;

    ProgressBase::update(
        &pool,
        object.id,
        date,
        weight_kg,
        energy_burnt,
        notes,
        user.id,
    )
    .await?;
    if redirect_to.is_some() {
        leptos_axum::redirect(&format!("/users/{username}/{date}"));
    }
    Ok(())
}

#[component]
pub fn ProgressUpdatePage() -> impl IntoView {
    let params = use_params_map();
    let username = move || get_username(&params);
    let date = move || get_date(&params);
    let resource = Resource::new(
        move || (username(), date()),
        |(username, date)| get_progress_detail(username, date),
    );

    let action = Action::<ProgressUpdate, _>::server();

    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || extract_other_errors(action_value, &["name"]);
    let non_field_errors = move || get_non_field_errors(action_value);

    let response = move || {
        resource.and_then(|data| {
            let weight = data
                .weight
                .map_or_else(|| "".to_string(), |x| format!("{:.2}", x));
            let energy_burnt = data
                .energy_burnt
                .map_or_else(|| "".to_string(), |x| x.to_string());
            let username = data.username.clone();
            let date = data.date.to_string();
            let notes = data.notes.clone().unwrap_or_default();

            view! {
                <ActionForm action>
                    <input type="hidden" name="username" value=username/>
                    <input type="hidden" name="redirect_to" value="somewhere"/>
                    <TextInputImproved action_value name="date" input_type="date" value=date/>
                    <NumberInput
                        action_value
                        name="weight_kg"
                        label="Weight (kg)"
                        step="0.01"
                        placeholder="Enter your weight in kg"
                        value=weight
                    />
                    <NumberInput
                        action_value
                        name="energy_burnt"
                        label="Energy Burnt (kcal)"
                        value=energy_burnt
                        placeholder="Enter energy burnt in kcal"
                    />
                    <TextInputImproved
                        action_value
                        name="notes"
                        value=notes
                        placeholder="Enter any relevant notes"
                    />
                    <SubmitButton loading=action_loading label="Update Progress"/>
                </ActionForm>
            }
        })
    };

    view! {
        <DetailPageTemplate title="Edit Progress">
            <div class="mb-4 text-red-500 font-bold">{action_error}</div>
            <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>
            <Transition fallback=LoadingComponent>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors=errors/> }
                }>{response}</ErrorBoundary>
            </Transition>
        </DetailPageTemplate>
    }
}
