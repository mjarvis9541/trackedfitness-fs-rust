use std::collections::HashSet;

use leptos::*;
use leptos_router::*;

use crate::component::button::Button;
use crate::component::icon::IconFilePlus;
use crate::component::input::TextInput;
use crate::util::param::get_username;
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};

#[cfg(feature = "ssr")]
use crate::{
    auth::model::User, auth::service::get_request_user, diet::model::Diet, error::Error,
    meal::model::Meal, meal_food::model::MealFoodModel, setup::get_pool,
    util::server::parse_uuids_from_strings,
};

#[server]
pub async fn save_to_meal(
    username: String,
    meal_name: String,
    items: Option<HashSet<String>>,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let items = items.ok_or_else(|| ServerFnError::new("Nothing selected"))?;
    items.len().ge(&10).then(|| ()).ok_or(ServerFnError::new(
        "You are only allowed to create 10 items at a time.",
    ))?;
    let target_user = User::get_by_username(&pool, &username)
        .await?
        .ok_or(Error::NotFound)?;
    Meal::can_create(&target_user.clone().into()).await?;
    let diet_uuids = parse_uuids_from_strings(&items)?;
    Meal::validate(&meal_name)?;
    let diet_food = Diet::all_by_ids(&pool, &diet_uuids).await?;
    diet_food
        .is_empty()
        .then(|| ())
        .ok_or(ServerFnError::new("No diet logs found."))?;
    let meal = Meal::create(&pool, target_user.id, &meal_name, user.id).await?;
    MealFoodModel::bulk_create_from_diet(&pool, meal.id, &diet_food, user.id).await?;
    leptos_axum::redirect(&format!("/food/meals/{}", meal.id));
    Ok(())
}

#[component]
pub fn DietToMealForm(
    checked_items: RwSignal<HashSet<String>>,
    action: Action<SaveToMeal, Result<(), ServerFnError>>,
) -> impl IntoView {
    let params = use_params_map();
    let username = move || get_username(&params);

    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        if let Ok(mut data) = SaveToMeal::from_event(&ev) {
            data.items = Some(checked_items());
            checked_items.update(|v| v.clear());
            action.dispatch(data)
        }
    };

    let action_value = action.value();
    let action_error = move || extract_other_errors(action_value, &["name"]);
    let non_field_errors = move || get_non_field_errors(action_value);

    view! {
        <div>
            <div class="mb-4 text-red-500 font-bold">{action_error}</div>
            <div class="mb-4 text-red-500 font-bold">{non_field_errors}</div>
            <ActionForm action on:submit=handle_submit>
                <input type="hidden" name="items" value=""/>
                <input type="hidden" name="username" value=username/>
                <TextInput action_value name="name" placeholder="Enter meal name"/>
                <div class="flex justify-end">
                    <Button label="Save as Meal">
                        <IconFilePlus/>
                    </Button>
                </div>
            </ActionForm>
        </div>
    }
}
