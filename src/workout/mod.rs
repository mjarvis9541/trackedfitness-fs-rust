pub mod add_exercise_page;
pub mod component;
pub mod create_page;
pub mod day_page;
pub mod delete_page;
pub mod detail_page;
pub mod layout;
pub mod model;
pub mod multi_create;
#[cfg(feature = "ssr")]
pub mod permission;
#[cfg(feature = "ssr")]
pub mod repository_impl;
pub mod router;
// pub mod _sidebar;
pub mod update_page;
pub mod week_component;
pub mod week_navigation;
