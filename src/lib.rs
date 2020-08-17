#![recursion_limit = "256"]
mod api;
mod app;
mod components;
mod pages;
mod route;
mod types;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::initialize();
    App::<app::App>::new().mount_to_body();
}
