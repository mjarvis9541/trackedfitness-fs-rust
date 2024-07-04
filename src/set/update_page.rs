use leptos::*;
use leptos_router::*;

use rust_decimal::Decimal;
use uuid::Uuid;

use super::detail_page::get_set_detail;
use crate::auth::context::CanEditContext;
use crate::component::button::{Button, SubmitButton};
use crate::component::icon::{Chevron, IconCheck};
use crate::component::input::NumberInput;
use crate::component::template::{DetailPageTemplate, ErrorComponent, LoadingComponent};
use crate::util::validation_error::{extract_other_errors, get_non_field_errors};
use crate::workout::model::SetQueryWithPrevious;
use crate::workout::router::SetDetailParam;

#[cfg(feature = "ssr")]
use crate::{
    auth::service::get_request_user, error::Error, exercise::model::ExerciseModel,
    set::model::SetModel, setup::get_pool, workout::model::WorkoutBase,
};

#[server(endpoint = "set-order-increment")]
pub async fn set_order_update(set_id: Uuid, order: i32) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let set = SetModel::get_by_id(&pool, set_id)
        .await?
        .ok_or(Error::NotFound)?;

    let new_order = order + set.order;
    if new_order <= 1 {
        return Err(ServerFnError::new("must be a postivie number"));
    }
    SetModel::update_order(&pool, set.id, new_order, user.id).await?;
    Ok(())
}

#[component]
pub fn SetOrderUpdateForm(id: String, increment: bool) -> impl IntoView {
    let action = expect_context::<Action<SetOrderUpdate, Result<(), ServerFnError>>>();
    let order = if increment { 1 } else { -1 };
    let direction = move || if increment { "down" } else { "up" };
    view! {
        <ActionForm action class="contents">
            <input type="hidden" name="set_id" value=id/>
            <input type="hidden" name="order" value=order/>
            <Button>
                <Chevron direction=direction()/>
            </Button>
        </ActionForm>
    }
}

#[server(endpoint = "set-update")]
pub async fn set_update(
    set_id: Uuid,
    order: i32,
    weight: Decimal,
    reps: i32,
    rest: i32,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let set = SetModel::get_by_id(&pool, set_id)
        .await?
        .ok_or(Error::NotFound)?;
    let exercise = ExerciseModel::get_by_id(&pool, set.exercise_id)
        .await?
        .ok_or(Error::NotFound)?;
    let _workout = WorkoutBase::get_by_id(&pool, exercise.workout_id)
        .await?
        .ok_or(Error::NotFound)?;

    SetModel::validate(order, weight, reps, rest)?;
    SetModel::update(
        &pool,
        set.id,
        exercise.id,
        order,
        weight,
        reps,
        rest,
        user.id,
    )
    .await?;
    Ok(())
}

#[component]
pub fn SetRowInput(
    name: &'static str,
    #[prop(default = name)] label: &'static str,
    #[prop(default = 1.0)] step: f64,
    #[prop(optional, into)] value: MaybeSignal<String>,
) -> impl IntoView {
    let current_user_context = expect_context::<CanEditContext>();

    let disabled = move || current_user_context.cant_edit();

    view! {
        <label class="relative w-full">
            <div class="flex absolute inset-y-0 right-0 items-center pr-2 text-gray-400 pointer-events-none select-none">
                {label}
            </div>
            <input
                type="number"
                autocomplete="off"
                name=name
                step=step
                value=value
                disabled=disabled
                onchange="this.form.requestSubmit()"
                class="py-1.5 pr-14 pl-2 w-full bg-gray-50 border focus:border-blue-500 focus:ring-2 focus:ring-blue-500 focus:outline-none disabled:text-gray-800 disabled:bg-gray-300 disabled:opacity-50"
            />
        </label>
    }
}

#[component]
pub fn SetRowUpdateForm(data: SetQueryWithPrevious) -> impl IntoView {
    let action = expect_context::<Action<SetUpdate, Result<(), ServerFnError>>>();
    let set_id = data.set_id.to_string();
    let order = data.order.to_string();
    let reps = data.reps.to_string();
    let rest = data.rest;
    let weight = format!("{:.2}", data.weight);
    view! {
        <ActionForm action class="contents">
            <input type="hidden" name="set_id" value=set_id/>
            <input type="hidden" name="rest" value=rest/>
            <SetRowInput name="order" value=order/>
            <SetRowInput name="weight" value=weight step=0.1/>
            <SetRowInput name="reps" value=reps/>
            <Button>
                <IconCheck/>
            </Button>
        </ActionForm>
    }
}

#[component]
pub fn SetUpdatePage() -> impl IntoView {
    let params = use_params::<SetDetailParam>();
    let set_id = move || params.with(|q| q.as_ref().map_or_else(|_| Uuid::default(), |q| q.set_id));

    let action = Action::<SetUpdate, _>::server();
    let action_loading = action.pending();
    let action_value = action.value();
    let action_error = move || extract_other_errors(action_value, &["name"]);
    let non_field_errors = move || get_non_field_errors(action_value);

    let resource = Resource::new(set_id, get_set_detail);

    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let data = SetUpdate::from_event(&ev);
        if let Ok(data) = data {
            action.dispatch(data)
        }
    };

    let response = move || {
        resource.and_then(|data| {
            let id = data.id.to_string();
            let order = data.order.to_string();
            let weight = data.weight.to_string();
            let reps = data.reps.to_string();
            let rest = data.rest.unwrap_or_default().to_string();

            view! {
                <ActionForm action on:submit=handle_submit>
                    <input type="hidden" name="set_id" value=id/>
                    <NumberInput action_value name="order" value=order label="Set No."/>
                    <NumberInput action_value name="weight" step="0.01" min="0" value=weight/>
                    <NumberInput action_value name="reps" step="1" min="0" max="100" value=reps/>
                    <NumberInput action_value name="rest" step="1" value=rest/>
                    <SubmitButton loading=action_loading label="Update Set"/>
                </ActionForm>
            }
        })
    };

    view! {
        <DetailPageTemplate title="Edit Set">
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
