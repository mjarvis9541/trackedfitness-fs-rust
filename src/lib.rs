mod admin;
pub mod app;
mod auth;
mod brand;
mod chart;
mod component;
mod diet;
mod diet_target;
mod error;
mod error_extract;
mod exercise;
mod follower;
mod food;
mod meal;
mod meal_food;
mod meal_of_day;
mod movement;
mod muscle_group;
mod profile;
mod progress;
mod set;
mod summary;
mod user;
mod user_block;
mod user_setting;
mod user_statistic;
mod util;
mod web;
mod workout;

#[cfg(feature = "ssr")]
pub mod config;
#[cfg(feature = "ssr")]
pub mod middleware;
#[cfg(feature = "ssr")]
pub mod setup;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
