use leptos::*;
use leptos_router::*;

use crate::exercise::create_page::ExerciseCreate;
use crate::exercise::create_with_set_form::ExerciseSetCreate;
use crate::exercise::delete_page::ExerciseDelete;
use crate::set::create_form::SetCreate;
use crate::set::delete_page::SetDelete;
use crate::set::update_page::SetUpdate;
// use crate::util::param::get_username;
use crate::workout::create_page::WorkoutCreate;
use crate::workout::delete_page::WorkoutDelete;
// use crate::workout::sidebar::{get_workout_sidebar, WorkoutSidebar};

#[component]
pub fn WorkoutLayout() -> impl IntoView {
    // let params = use_params_map();
    // let username = move || get_username(&params);

    let action_workout_create = Action::<WorkoutCreate, _>::server();
    let action_workout_delete = Action::<WorkoutDelete, _>::server();
    let action_exercise_create = Action::<ExerciseCreate, _>::server();
    let action_exercise_set_create = Action::<ExerciseSetCreate, _>::server();
    let action_exercise_delete = Action::<ExerciseDelete, _>::server();
    let action_set_create = Action::<SetCreate, _>::server();
    let action_set_update = Action::<SetUpdate, _>::server();
    let action_set_delete = Action::<SetDelete, _>::server();

    // let sidebar_resource: SidebarResource = Resource::new(
    //     move || {
    //         (
    //             username(),
    //             action_exercise_create.version().get(),
    //             action_exercise_set_create.version().get(),
    //             action_exercise_delete.version().get(),
    //             action_workout_create.version().get(),
    //             action_workout_delete.version().get(),
    //             action_set_create.version().get(),
    //             action_set_update.version().get(),
    //             action_set_delete.version().get(),
    //         )
    //     },
    //     |(username, ..)| get_workout_sidebar(username),
    // );

    // provide_context(sidebar_resource);
    provide_context(action_workout_create);
    provide_context(action_workout_delete);
    provide_context(action_exercise_create);
    provide_context(action_exercise_set_create);
    provide_context(action_exercise_delete);
    provide_context(action_set_create);
    provide_context(action_set_update);
    provide_context(action_set_delete);

    view! { <Outlet/> }
}

// <main class="grid grid-cols-4 lg:grid-cols-12">
// <section class="hidden col-span-3 p-2 bg-white lg:block">
//     <WorkoutSidebar/>
// </section>
// <section class="col-span-4 bg-white lg:col-span-9">
//     <Outlet/>
// </section>
// </main>
