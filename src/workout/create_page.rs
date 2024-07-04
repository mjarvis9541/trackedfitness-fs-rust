use leptos::*;
use leptos_router::*;

use chrono::prelude::*;

use crate::component::button::Button;
use crate::component::icon::IconFilePlus;
use crate::util::param::{get_date, get_username};

#[cfg(feature = "ssr")]
use crate::{
    auth::model::User, auth::service::get_request_user, error::Error, setup::get_pool,
    workout::model::WorkoutBase,
};

#[server]
pub async fn workout_create(username: String, date: NaiveDate) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    let target_user = User::get_by_username(&pool, &username)
        .await?
        .ok_or(Error::NotFound)?;

    WorkoutBase::can_create(&target_user, &user).await?;

    WorkoutBase::create(&pool, target_user.id, date, user.id).await?;
    Ok(())
}

#[component]
pub fn WorkoutCreateForm() -> impl IntoView {
    let params = use_params_map();
    let username = move || get_username(&params);
    let date = move || get_date(&params).to_string();

    let action = expect_context::<Action<WorkoutCreate, Result<(), ServerFnError>>>();
    view! {
        <ActionForm action class="contents">
            <input type="hidden" name="username" value=username/>
            <input type="hidden" name="date" value=date/>
            <Button label="New Workout">
                <IconFilePlus/>
            </Button>
        </ActionForm>
    }
}
