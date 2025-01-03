pub mod app;
pub mod components;
pub mod sandbox;
pub mod scoresheet;

#[cfg(feature = "ssr")]
pub mod file_server;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
