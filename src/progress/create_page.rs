use chrono::prelude::*;
use leptos::*;
use leptos_router::*;
use rust_decimal::Decimal;

use crate::component::button::SubmitButton;
use crate::component::input::{NumberInput, TextInput};
use crate::component::template::DetailPageTemplate;
use crate::util::param::{get_date, get_username};
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{
    auth::model::User, auth::service::get_request_user, error::Error, progress::model::Progress,
    setup::get_pool,
};

#[server(endpoint = "progress-create")]
pub async fn progress_create(
    username: String,
    date: NaiveDate,
    weight_kg: Option<Decimal>,
    energy_burnt: Option<i32>,
    notes: Option<String>,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let target_user = User::get_by_username(&pool, &username)
        .await?
        .ok_or(Error::NotFound)?;

    Progress::can_create(&user, target_user.id).await?;
    Progress::validate(date, weight_kg, energy_burnt, notes.clone())?;

    Progress::create(
        &pool,
        target_user.id,
        date,
        weight_kg,
        energy_burnt,
        notes,
        user.id,
    )
    .await?;

    leptos_axum::redirect(&format!("/users/{}/{}", target_user.username, date));
    Ok(())
}

#[component]
pub fn ProgressCreatePage() -> impl IntoView {
    let params = use_params_map();
    let username = move || get_username(&params);

    let query = use_query_map();
    let create_for_date = move || get_date(&query).to_string();

    let action = Action::<ProgressCreate, _>::server();
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || {
        extract_other_errors(
            action_value,
            &[
                "non_field_errors",
                "date",
                "username",
                "weight_kg",
                "energy_burnt",
                "notes",
            ],
        )
    };
    let non_field_errors = move || get_non_field_errors(action_value);

    view! {
        <DetailPageTemplate title="Log Progress">

            <div class="mb-4 text-red-500 font-bold">{action_error}</div>

            <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>
            <ActionForm action>
                <input type="hidden" name="username" value=username/>
                <TextInput action_value input_type="date" name="date" value=create_for_date()/>
                <NumberInput
                    action_value
                    name="weight_kg"
                    step="0.01"
                    label="Weight (kg)"
                    placeholder="Enter your weight in kg"
                />
                <NumberInput
                    action_value
                    name="energy_burnt"
                    label="Energy Burnt (kcal)"
                    placeholder="Enter energy burnt in kcal"
                />
                <TextInput
                    action_value
                    name="notes"
                    placeholder="Enter any relevant notes for the day"
                />

                <SubmitButton loading=action_loading label="Log Progress"/>
            </ActionForm>
        </DetailPageTemplate>
    }
}
