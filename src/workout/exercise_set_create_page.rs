use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use chrono::prelude::*;
use rust_decimal::prelude::ToPrimitive;
use uuid::Uuid;

use crate::component::input::FilterInput;
use crate::component::paginator::Paginator;
use crate::component::template::{ErrorComponent, ListLoadingComponent, ListNotFoundComponent};
use crate::exercise::create_page::ExerciseCreateForm;
use crate::exercise::with_set_create_page::ExerciseSetCreateForm;
use crate::movement::model::MovementWithLatestWeight;
use crate::muscle_group::select::{get_muscle_group_filter, MuscleGroupFilter};
use crate::util::datetime::DATE_FORMAT_LONG;
use crate::util::param::{extract_page, extract_param, extract_size};
use crate::workout::multi_create::all_movement_with_latest_weight;
use crate::workout::router::WorkoutDetailParam;

#[component]
pub fn ExerciseSetCreatePage() -> impl IntoView {
    let params = use_params::<WorkoutDetailParam>();
    let username =
        move || params.with(|p| p.as_ref().map(|p| p.username.clone()).unwrap_or_default());
    let date = move || {
        params.with(|p| {
            p.as_ref()
                .map_or_else(|_| Utc::now().date_naive(), |p| p.date)
        })
    };
    let workout_id = move || {
        params.with(|p| {
            p.as_ref()
                .map_or_else(|_| Uuid::default(), |p| p.workout_id)
        })
    };
    let action_redirect_href = move || format!("/users/{}/workouts/{}", username(), date());
    let date_title = move || date().format(DATE_FORMAT_LONG).to_string();

    let query = use_query_map();
    let search = move || extract_param(&query, "search");
    let muscle_group = move || extract_param(&query, "muscle_group");
    let order = move || extract_param(&query, "order");
    let size = move || extract_size(&query);
    let page = move || extract_page(&query);

    let muscle_group_filter = Resource::once(get_muscle_group_filter);
    provide_context(muscle_group_filter);

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
                    .map(|data| {
                        view! {
                            <ExerciseSetCreateListItem
                                data=data
                                workout_id=workout_id()
                                redirect_to=action_redirect_href()
                            />
                        }
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
        <Title text="Add Exercise"/>
        <main class="p-4 m-4 bg-white border">

            <header class="mb-4">
                <h1 class="text-xl font-bold">"Add Exercise"</h1>
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

            <section class="grid grid-cols-7">
                <ExerciseSetCreateListHeader/>
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
pub fn ExerciseSetCreateListHeader() -> impl IntoView {
    view! {
        <div class="flex col-span-2 items-center p-2 font-bold border-b">"Exercise"</div>
        <div class="flex items-center p-2 font-bold border-b">"Weight"</div>
        <div class="flex items-center p-2 font-bold border-b">"Sets"</div>
        <div class="flex items-center p-2 font-bold border-b">"Reps"</div>
        <div class="flex items-center p-2 font-bold border-b">"Add"</div>
        <div class="flex items-center p-2 font-bold border-b">"Add Without Sets"</div>
    }
}

#[component]
pub fn ExerciseSetCreateListItem<'a>(
    data: &'a MovementWithLatestWeight,
    workout_id: Uuid,
    redirect_to: String,
) -> impl IntoView {
    let workout_id = workout_id.to_string();
    let movement_id = data.movement_id.to_string();

    let weight = data.latest_exercise_weight.to_i32().unwrap_or_default();
    let sets = data.latest_exercise_sets as i32;
    let reps = data.latest_exercise_reps as i32;

    let redirect_to_cloned = redirect_to.clone();
    let workout_id_cloned = workout_id.clone();
    let movement_id_cloned = movement_id.clone();

    view! {
        <div class="contents group">
            <div class="col-span-2 p-2 group-hover:bg-gray-200 group-odd:bg-gray-50 truncate">
                <h2>
                    <a class="hover:underline" href=format!("/exercises/{}", data.movement_slug)>
                        {&data.movement_name}
                    </a>
                </h2>
                <a
                    class="block text-xs text-gray-500 hover:underline"
                    href=format!("/muscle-groups/{}", data.muscle_group_slug)
                >
                    {&data.muscle_group_name}
                </a>
            </div>
            <ExerciseSetCreateForm redirect_to workout_id movement_id weight sets reps/>
            <ExerciseCreateForm
                redirect_to=redirect_to_cloned
                workout_id=workout_id_cloned
                movement_id=movement_id_cloned
            />
        </div>
    }
}
