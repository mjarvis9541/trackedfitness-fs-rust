use leptos::*;
use leptos_router::*;

use rust_decimal::Decimal;
use uuid::Uuid;

use crate::brand::select::{get_brand_select, BrandSelect};
use crate::component::button::SubmitButton;
use crate::component::input::{NumberInput, TextInput};
use crate::component::select::FieldSelect;
use crate::component::template::{DetailPageTemplate, ErrorComponent, LoadingComponent};
use crate::util::param::get_slug;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

use super::data_measurement::DataMeasurement;
use super::detail_page::get_food_detail;

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user,
    brand::model::Brand,
    error::Error,
    food::model::{Food, FoodQuery},
    setup::get_pool,
};

#[server(endpoint = "food-update")]
pub async fn food_update(
    slug: String,
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

    let food = Food::get_by_slug(&pool, &slug)
        .await?
        .ok_or(Error::NotFound)?;
    food.can_update(&user).await?;

    let brand = Brand::get_by_id(&pool, brand_id)
        .await?
        .ok_or(Error::NotFound)?;
    FoodQuery::validate(
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

    let object = Food::update(
        &pool,
        food.id,
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

    leptos_axum::redirect(&format!("/food/{}", object.slug));
    Ok(())
}

#[component]
pub fn FoodUpdatePage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || get_slug(&params);

    let action = Action::<FoodUpdate, _>::server();
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

    let resource = Resource::new(slug, get_food_detail);

    let response = move || {
        resource.and_then(|data| {
            let slug = data.slug.clone();
            let name = data.name.clone();
            let serving = data.data_measurement.to_string();
            let energy = data.energy.to_string();
            let fat = format!("{:.2}", data.fat);
            let saturates = format!("{:.2}", data.saturates);
            let carbohydrate = format!("{:.2}", data.carbohydrate);
            let sugars = format!("{:.2}", data.sugars);
            let fibre = format!("{:.2}", data.fibre);
            let protein = format!("{:.2}", data.protein);
            let salt = format!("{:.2}", data.salt);
            let brand_id = data.brand_id;

            let serving_options = DataMeasurement::to_form_options();
            view! {
                <ActionForm action>
                    <input type="hidden" name="slug" value=slug/>
                    <TextInput
                        name="name"
                        action_value
                        value=name
                        placeholder="Enter food name, e.g. Chicken Breast"
                    />
                    <BrandSelect selected=brand_id/>
                    <FieldSelect name="serving" value=serving options=serving_options/>

                    <NumberInput
                        placeholder="0"
                        name="energy"
                        value=energy
                        label="Energy (kcal)"
                        action_value
                    />
                    <NumberInput
                        placeholder="0.0"
                        name="fat"
                        value=fat
                        step="0.01"
                        label="Fat (g)"
                        action_value
                    />
                    <NumberInput
                        placeholder="0.0"
                        name="saturates"
                        value=saturates
                        step="0.01"
                        label="Saturates (g)"
                        action_value
                    />
                    <NumberInput
                        placeholder="0.0"
                        name="carbohydrate"
                        value=carbohydrate
                        step="0.01"
                        label="Carbohydrate (g)"
                        action_value
                    />
                    <NumberInput
                        placeholder="0.0"
                        name="sugars"
                        label="Sugars (g)"
                        value=sugars
                        step="0.01"
                        action_value
                    />
                    <NumberInput
                        placeholder="0.0"
                        name="fibre"
                        value=fibre
                        step="0.01"
                        label="Fibre (g)"
                        action_value
                    />
                    <NumberInput
                        placeholder="0.00"
                        name="protein"
                        value=protein
                        step="0.01"
                        label="Protein (g)"
                        action_value
                    />
                    <NumberInput name="salt" value=salt step="0.01" label="Salt (g)" action_value/>

                    <SubmitButton loading=action_loading label="Update Food"/>
                </ActionForm>
            }
        })
    };

    view! {
        <DetailPageTemplate title="Edit Food">
            <div class="mb-4 text-red-500 font-bold">{action_error}</div>
            <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>
            <Transition fallback=LoadingComponent>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorComponent errors/> }
                }>{response}</ErrorBoundary>
            </Transition>
        </DetailPageTemplate>
    }
}
