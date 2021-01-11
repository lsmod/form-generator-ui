#![recursion_limit = "1024"]
mod app;
extern crate strum;
#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate serde_derive;
extern crate askama;
extern crate inflector;
extern crate serde;
extern crate serde_json;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<app::App>();
    Ok(())
}
