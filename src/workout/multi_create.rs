use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use chrono::prelude::*;
use rust_decimal::prelude::*;
use uuid::Uuid;

use crate::component::button::Button;
use crate::component::icon::IconFilePlus;
use crate::component::input::{FilterInput, SetInput};
use crate::component::paginator::Paginator;
use crate::component::template::{ErrorComponent, ListLoadingComponent, ListNotFoundComponent};
use crate::movement::model::MovementWithLatestWeight;
use crate::muscle_group::select::{get_muscle_group_filter, MuscleGroupFilter};
use crate::util::datetime::DATE_FORMAT_LONG;
use crate::util::misc::ListResponse;
use crate::util::param::{extract_page, extract_param, extract_size, get_date, get_username};

#[cfg(feature = "ssr")]
use crate::{
    auth::model::User, auth::service::get_request_user, error::Error,
    exercise::model::ExerciseBase, movement::model::MovementQuery, set::model::SetModel,
    setup::get_pool, workout::model::WorkoutBase,
};

#[server]
pub async fn workout_exercise_set_create(
    username: String,
    date: NaiveDate,
    movement_id: Uuid,
    weight: Decimal,
    reps: i32,
    rest: i32,
    set_count: i32,
    redirect_to: Option<String>,
) -> Result<(), ServerFnError> {
    let user = get_request_user()?;
    let pool = get_pool()?;

    if !user.is_superuser && set_count > 10 {
        return Err(ServerFnError::new("Maximum of 10 sets to be created."));
    }
    let target_user = User::get_by_username(&pool, &username)
        .await?
        .ok_or(Error::NotFound)?;

    WorkoutBase::can_create(&target_user, &user).await?;
    let workout = WorkoutBase::create(&pool, target_user.id, date, user.id).await?;
    let exercise = ExerciseBase::create(&pool, workout.id, movement_id, user.id).await?;

    SetModel::bulk_create(&pool, exercise.id, weight, reps, rest, set_count, user.id).await?;
    if let Some(redirect_to) = redirect_to {
        leptos_axum::redirect(&redirect_to);
    }
    Ok(())
}

#[server]
pub async fn all_movement_with_latest_weight(
    username: String,
    search: String,
    muscle_group: String,
    order: String,
    size: i64,
    page: i64,
) -> Result<ListResponse<MovementWithLatestWeight>, ServerFnError> {
    get_request_user()?;
    let pool = get_pool()?;
    let count = MovementQuery::count(&pool, &search, &muscle_group).await?;
    let results = MovementWithLatestWeight::with_latest_weight(
        &pool,
        &username,
        &search,
        &muscle_group,
        &order,
        size,
        page,
    )
    .await?;
    Ok(ListResponse { results, count })
}

#[component]
pub fn WorkoutExerciseSetCreatePage() -> impl IntoView {
    let params = use_params_map();
    let username = move || get_username(&params);
    let date = move || get_date(&params);

    let date_str = move || date().to_string();
    let date_title = move || date().format(DATE_FORMAT_LONG).to_string();

    let query = use_query_map();
    let search = move || extract_param(&query, "search");
    let muscle_group = move || extract_param(&query, "muscle_group");
    let order = move || extract_param(&query, "order");
    let size = move || extract_size(&query);
    let page = move || extract_page(&query);

    let action = Action::<WorkoutExerciseSetCreate, _>::server();
    provide_context(action);

    let mg_resource = Resource::once(get_muscle_group_filter);
    provide_context(mg_resource);

    let resource = Resource::new(
        move || {
            (
                username(),
                search(),
                muscle_group(),
                order(),
                size(),
                page(),
            )
        },
        |(username, search, muscle_group, order, size, page)| {
            all_movement_with_latest_weight(username, search, muscle_group, order, size, page)
        },
    );

    let response = move || {
        resource.and_then(|data| {
            let results = &data.results;
            if data.count == 0 {
                view! { <ListNotFoundComponent/> }
            } else {
                results
                    .iter()
                    .map(|inner| {
                        view! { <WorkoutCreateListItem data=inner username=username() date=date_str()/> }
                    })
                    .collect_view()
            }
        })
    };

    let count = move || {
        resource.with(|res| {
            res.as_ref()
                .and_then(|data| data.as_ref().ok().map(|res| res.count))
        })
    };

    view! {
        <Title text="Create Workout"/>
        <main class="p-4 m-4 bg-white border">
            <header class="mb-4">
                <h1 class="text-xl font-bold">"Create Workout"</h1>
                <div>{date_title}</div>
            </header>

            <section class="flex flex-wrap gap-2 mb-4 lg:mb-2">
                <Form method="GET" action="" class="contents">
                    <input type="hidden" name="page" value=1/>
                    <input type="hidden" name="size" value=size/>
                    <FilterInput name="search" value=Signal::derive(search)/>
                    <MuscleGroupFilter selected=Signal::derive(muscle_group)/>
                </Form>
            </section>
            <section class="grid grid-cols-6">
                <WorkoutCreateListHeader/>
                <Transition fallback=ListLoadingComponent>
                    <ErrorBoundary fallback=|errors| {
                        view! { <ErrorComponent errors/> }
                    }>{response}</ErrorBoundary>
                </Transition>
            </section>
            <section class="flex-1">
                <Form method="GET" action="" class="contents">
                    <input type="hidden" name="search" value=search/>
                    <input type="hidden" name="muscle_group" value=muscle_group/>
                    <input type="hidden" name="order" value=order/>
                    <input type="hidden" name="page" value=page/>
                    <Transition>
                        <Paginator count/>
                    </Transition>
                </Form>
            </section>
        </main>
    }
}

#[component]
pub fn WorkoutCreateListHeader() -> impl IntoView {
    view! {
        <div class="flex col-span-2 items-center p-2 font-bold border-b">"Exercise"</div>
        <div class="flex items-center p-2 font-bold border-b">"Weight"</div>
        <div class="flex items-center p-2 font-bold border-b">"Sets"</div>
        <div class="flex items-center p-2 font-bold border-b">"Reps"</div>
        <div class="flex items-center p-2 font-bold border-b">"Add"</div>
    }
}

#[component]
pub fn WorkoutCreateListItem<'a>(
    data: &'a MovementWithLatestWeight,
    username: String,
    date: String,
) -> impl IntoView {
    let action = expect_context::<Action<WorkoutExerciseSetCreate, Result<(), ServerFnError>>>();
    let loading = action.pending();
    let redirect_to = format!("/users/{}/workouts/{}", username, date);
    let movement_id = data.movement_id.to_string();

    let weight = data.latest_exercise_weight.to_i32().unwrap_or_default();
    let sets = data.latest_exercise_sets as i32;
    let reps = data.latest_exercise_reps as i32;

    view! {
        <div class="contents group">
            <div class="col-span-2 p-2 group-hover:bg-gray-200 group-odd:bg-gray-50 truncate">
                <h2>
                    <a class="hover:underline" href=format!("/exercises/{}", data.movement_slug)>
                        {&data.movement_name}
                    </a>
                </h2>
                <p>
                    <a
                        class="text-xs text-gray-500 hover:underline"
                        href=format!("/muscle-groups/{}", data.muscle_group_slug)
                    >
                        {&data.muscle_group_name}
                    </a>
                </p>
            </div>
            <ActionForm action class="contents">
                <div class="p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
                    <SetInput name="weight" label="kg" value=weight/>
                </div>
                <div class="p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
                    <SetInput name="set_count" label="sets" value=sets/>
                </div>
                <div class="p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
                    <SetInput name="reps" label="reps" value=reps/>
                </div>
                <div class="flex justify-end items-center p-2 group-hover:bg-gray-200 group-odd:bg-gray-50">
                    <input type="hidden" name="redirect_to" value=redirect_to/>
                    <input type="hidden" name="username" value=username/>
                    <input type="hidden" name="date" value=date/>
                    <input type="hidden" name="movement_id" value=movement_id/>
                    <input type="hidden" name="rest" value="60"/>
                    <Button loading label="Add">
                        <IconFilePlus/>
                    </Button>
                </div>
            </ActionForm>

        </div>
    }
}
