use leptos::*;
use leptos_router::*;

use chrono::prelude::*;
use rust_decimal::Decimal;

use crate::component::button::SubmitButton;
use crate::component::input::{NumberInput, TextInput};
use crate::component::template::DetailPageTemplate;
use crate::util::param::{get_date, get_username};
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{
    auth::model::User,
    auth::service::get_request_user,
    diet_target::model::{DietTargetBase, DietTargetGramKg, DietTargetInput},
    error::Error,
    setup::get_pool,
};

#[server]
pub async fn diet_target_create(
    username: String,
    date: NaiveDate,
    weight: Decimal,
    protein_per_kg: Decimal,
    carbohydrate_per_kg: Decimal,
    fat_per_kg: Decimal,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let target_user = User::get_by_username(&pool, &username)
        .await?
        .ok_or(Error::NotFound)?;
    DietTargetBase::can_create(&user, target_user.id).await?;

    let data = DietTargetGramKg {
        user_id: target_user.id,
        date,
        weight,
        protein_per_kg,
        carbohydrate_per_kg,
        fat_per_kg,
    };
    data.validate()?;
    let database_input = DietTargetInput::from(data);
    let query = DietTargetBase::create(&pool, database_input, user.id).await?;

    leptos_axum::redirect(&format!(
        "/users/{}/diet-targets/{}",
        target_user.username, query.date
    ));
    Ok(())
}

#[component]
pub fn DietTargetCreatePage() -> impl IntoView {
    let params = use_params_map();
    let username = move || get_username(&params);

    let query = use_query_map();
    let create_for_date = move || get_date(&query).to_string();

    let action = Action::<DietTargetCreate, _>::server();
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || {
        extract_other_errors(
            action_value,
            &[
                "non_field_errors",
                "id",
                "username",
                "date",
                "weight",
                "protein_per_kg",
                "carbohydrate_per_kg",
                "fat_per_kg",
            ],
        )
    };
    let non_field_errors = move || get_non_field_errors(action_value);

    view! {
        <DetailPageTemplate title="New Diet Target">

            <div class="mb-4 text-red-500 font-bold">{action_error}</div>
            <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>

            <ActionForm action>
                <input type="hidden" name="username" value=username/>
                <TextInput
                    action_value
                    input_type="date"
                    name="date"
                    value=create_for_date()
                />
                <NumberInput
                    action_value
                    step="0.01"
                    name="weight"
                    label="Weight (kg)"
                    placeholder="60.0"
                />
                <NumberInput
                    action_value
                    step="0.01"
                    name="protein_per_kg"
                    label="Protein (grams per kg)"
                    placeholder="2.50"
                />
                <NumberInput
                    action_value
                    step="0.01"
                    name="carbohydrate_per_kg"
                    label="Carbohydrate (grams per kg)"
                    placeholder="5.00"
                />
                <NumberInput
                    action_value
                    step="0.01"
                    name="fat_per_kg"
                    label="Fat (grams per kg)"
                    placeholder="1.00"
                />
                <SubmitButton loading=action_loading label="Create Diet Target"/>
            </ActionForm>
        </DetailPageTemplate>
    }
}
