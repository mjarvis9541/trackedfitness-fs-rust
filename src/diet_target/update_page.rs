use leptos::*;
use leptos_router::*;

use chrono::prelude::*;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::component::button::SubmitButton;
use crate::component::input::{NumberInput, TextInputImproved};
use crate::component::template::{DetailPageTemplate, ErrorComponent, LoadingComponent};
use crate::util::param::{get_date, get_username};
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

use super::detail_page::get_diet_target_detail;
use super::model::DietTarget;

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user,
    diet_target::model::{DietTargetBase, DietTargetGramKg, DietTargetInput},
    error::Error,
    setup::get_pool,
};

#[server(endpoint = "diet-target-update")]
pub async fn diet_target_update(
    id: Uuid,
    username: String,
    date: NaiveDate,
    weight: Decimal,
    protein_per_kg: Decimal,
    carbohydrate_per_kg: Decimal,
    fat_per_kg: Decimal,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let object = DietTargetBase::get_by_id(&pool, id)
        .await?
        .ok_or(Error::NotFound)?;
    object.can_update(&user).await?;

    let data = DietTargetGramKg {
        user_id: object.user_id,
        date,
        weight,
        protein_per_kg,
        carbohydrate_per_kg,
        fat_per_kg,
    };
    data.validate()?;
    let database_input = DietTargetInput::from(data);
    let query = DietTargetBase::update(&pool, object.id, database_input, user.id).await?;

    leptos_axum::redirect(&format!("/users/{username}/diet-targets/{}", query.date));
    Ok(())
}

#[component]
pub fn DietTargetUpdatePage() -> impl IntoView {
    let params = use_params_map();
    let username = move || get_username(&params);
    let date = move || get_date(&params);

    let resource = Resource::new(
        move || (username(), date()),
        |(username, date)| get_diet_target_detail(username, date),
    );

    let action = Action::<DietTargetUpdate, _>::server();

    let response =
        move || resource.and_then(|data| view! { <DietTargetForm data=data.clone() action/> });

    view! {
        <DetailPageTemplate title="Edit Diet Target">
            <Transition fallback=LoadingComponent>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors=errors/> }
                }>{response}</ErrorBoundary>
            </Transition>
        </DetailPageTemplate>
    }
}

#[component]
pub fn DietTargetForm(
    data: DietTarget,
    action: Action<DietTargetUpdate, Result<(), ServerFnError>>,
) -> impl IntoView {
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
        <div class="mb-4 text-red-500 font-bold">{action_error}</div>
        <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>
        <ActionForm action>
            <input type="hidden" name="id" value=data.id.to_string()/>
            <input type="hidden" name="username" value=data.username/>
            <TextInputImproved
                action_value
                input_type="date"
                name="date"
                value=data.date.to_string()
            />
            <NumberInput
                action_value
                name="weight"
                step="0.01"
                label="Weight (kg)"
                value=format!("{:.2}", data.weight)
            />
            <NumberInput
                action_value
                step="0.01"
                name="protein_per_kg"
                label="Protein (grams per kg)"
                placeholder="2.50"
                value=format!("{:.2}", data.protein_per_kg)
            />
            <NumberInput
                action_value
                step="0.01"
                name="carbohydrate_per_kg"
                label="Carbohydrate (grams per kg)"
                placeholder="5.00"
                value=format!("{:.2}", data.carbohydrate_per_kg)
            />
            <NumberInput
                action_value
                step="0.01"
                name="fat_per_kg"
                label="Fat (grams per kg)"
                placeholder="1.00"
                value=format!("{:.2}", data.fat_per_kg)
            />
            <SubmitButton loading=action_loading label="Update Diet Target"/>
        </ActionForm>
    }
}
