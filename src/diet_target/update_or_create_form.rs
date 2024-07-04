use std::collections::HashSet;

use leptos::*;
use leptos_router::*;

use rust_decimal::Decimal;

use crate::component::button::SubmitButton;
use crate::component::input::NumberInput;
use crate::util::param::get_username;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[cfg(feature = "ssr")]
use {
    crate::{
        auth::model::User,
        auth::service::get_request_user,
        diet_target::model::{DietTarget, DietTargetBase, DietTargetGramKg, DietTargetInput},
        error::Error,
        setup::get_pool,
        util::server::parse_dates_from_strings,
    },
    chrono::prelude::*,
};

#[server(endpoint = "diet-target-bulk-update-or-create")]
pub async fn diet_target_bulk_update_or_create(
    username: String,
    date_range: Option<HashSet<String>>,
    weight: Decimal,
    protein_per_kg: Decimal,
    carbohydrate_per_kg: Decimal,
    fat_per_kg: Decimal,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;
    let some_date_range = date_range.ok_or_else(|| ServerFnError::new("Nothing selected"))?;
    if some_date_range.len() > 10 {
        return Err(ServerFnError::new(
            "You are only allowed to create 10 items at a time.",
        ));
    }
    let date_list = parse_dates_from_strings(&some_date_range)?;
    let target_user = User::get_by_username(&pool, &username)
        .await?
        .ok_or(Error::NotFound)?;
    DietTarget::can_create(&user, target_user.id).await?;

    let data = DietTargetGramKg {
        user_id: target_user.id,
        date: Utc::now().date_naive(),
        weight,
        protein_per_kg,
        carbohydrate_per_kg,
        fat_per_kg,
    };
    data.validate()?;
    let database_input = DietTargetInput::from(data);

    DietTargetBase::bulk_create_update(&pool, database_input, &date_list, user.id).await?;
    Ok(())
}

#[component]
pub fn DietTargetBulkUpdateOrCreateForm(
    checked_items: RwSignal<HashSet<String>>,
    action: Action<DietTargetBulkUpdateOrCreate, Result<(), ServerFnError>>,
) -> impl IntoView {
    let params = use_params_map();
    let username = move || get_username(&params);

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

    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        if let Ok(mut data) = DietTargetBulkUpdateOrCreate::from_event(&ev) {
            data.date_range = Some(checked_items());
            checked_items.update(|v| v.clear());
            action.dispatch(data)
        }
    };

    view! {
        <div class="mb-4 text-red-500 font-bold">{action_error}</div>
        <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>
        <ActionForm action on:submit=handle_submit class="contents">
            <input type="hidden" name="username" value=username/>
            <input type="hidden" name="date_range" value=""/>
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
            <SubmitButton loading=action_loading label="Create/Update Diet Targets"/>
        </ActionForm>
    }
}
