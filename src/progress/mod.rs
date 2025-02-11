pub mod component;
pub mod create_page;
pub mod delete_page;
pub mod detail_page;
pub mod detail_table;
pub mod list_page;
pub mod model;
#[cfg(feature = "ssr")]
pub mod permission;
#[cfg(feature = "ssr")]
pub mod repository_impl;
pub mod update_page;
#[cfg(feature = "ssr")]
pub mod validate;
