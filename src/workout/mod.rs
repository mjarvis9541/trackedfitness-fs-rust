pub mod component;
pub mod create_page;
pub mod day_page;
pub mod delete_page;
pub mod detail_page;
pub mod exercise_set_create_page;
// pub mod from_plan_create_page;
pub mod layout;
pub mod model;
pub mod multi_create;
#[cfg(feature = "ssr")]
pub mod permission;
#[cfg(feature = "ssr")]
pub mod repository_impl;
pub mod router;
pub mod sidebar;
pub mod update_page;
pub mod week_page;
