use leptos::*;
use leptos_router::*;

use rust_decimal::Decimal;
use uuid::Uuid;

use crate::brand::select::{get_brand_select, BrandSelect};
use crate::component::button::SubmitButton;
use crate::component::template::DetailPageTemplate;

use crate::component::input::{NumberInput, TextInputImproved};
use crate::component::select::FieldSelectB;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user, brand::model::Brand, error::Error, food::model::Food,
    setup::get_pool,
};

#[server(endpoint = "food-create")]
pub async fn food_create(
    name: String,
    brand_id: Uuid,
    serving: String,
    energy: i32,
    fat: Decimal,
    saturates: Decimal,
    carbohydrate: Decimal,
    sugars: Decimal,
    fibre: Decimal,
    protein: Decimal,
    salt: Decimal,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let brand = Brand::get_by_id(&pool, brand_id)
        .await?
        .ok_or(Error::NotFound)?;
    Food::validate(
        &name,
        &serving,
        energy,
        fat,
        saturates,
        carbohydrate,
        sugars,
        fibre,
        protein,
        salt,
    )?;

    let object = Food::create(
        &pool,
        &name,
        brand.id,
        &brand.name,
        serving,
        energy,
        fat,
        saturates,
        carbohydrate,
        sugars,
        fibre,
        protein,
        salt,
        user.id,
    )
    .await?;

    leptos_axum::redirect(&format!("/food/{}", object));
    Ok(())
}

#[component]
pub fn FoodCreatePage() -> impl IntoView {
    let action = Action::<FoodCreate, _>::server();
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || {
        extract_other_errors(
            action_value,
            &[
                "name",
                "serving",
                "brand_id",
                "energy",
                "fat",
                "saturates",
                "carbohydrate",
                "sugars",
                "fibre",
                "protein",
                "salt",
            ],
        )
    };
    let non_field_errors = move || get_non_field_errors(action_value);

    let brand_resource = Resource::once(get_brand_select);
    provide_context(brand_resource);
    let serving_options = vec![
        ("", "Select"),
        ("g", "100g"),
        ("ml", "100ml"),
        ("srv", "1 Serving"),
    ];
    view! {
        <DetailPageTemplate title="New Food">
            <div class="mb-4 text-red-500 font-bold">{action_error}</div>
            <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>

            <ActionForm action>
                <TextInputImproved
                    action_value
                    name="name"
                    placeholder="Enter food name, e.g. Chicken Breast"
                />
                <BrandSelect/>
                <FieldSelectB name="serving" options=serving_options/>

                <NumberInput action_value placeholder="0" name="energy" label="Energy (kcal)"/>
                <NumberInput action_value placeholder="0.0" name="fat" step="0.01" label="Fat (g)"/>
                <NumberInput
                    action_value
                    placeholder="0.0"
                    name="saturates"
                    step="0.01"
                    label="Saturates (g)"
                />
                <NumberInput
                    action_value
                    placeholder="0.0"
                    name="carbohydrate"
                    step="0.01"
                    label="Carbohydrate (g)"
                />
                <NumberInput
                    action_value
                    placeholder="0.0"
                    name="sugars"
                    label="Sugars (g)"
                    step="0.01"
                />
                <NumberInput
                    action_value
                    placeholder="0.0"
                    name="fibre"
                    step="0.01"
                    label="Fibre (g)"
                />
                <NumberInput
                    action_value
                    placeholder="0.0"
                    name="protein"
                    step="0.01"
                    label="Protein (g)"
                />
                <NumberInput
                    action_value
                    placeholder="0.00"
                    name="salt"
                    step="0.01"
                    label="Salt (g)"
                />

                <SubmitButton loading=action_loading label="Create Food"/>

            </ActionForm>
        </DetailPageTemplate>
    }
}
