pub mod app;
pub mod components;
pub mod content;
pub mod error_template;
pub mod pages;

#[cfg(feature = "ssr")]
pub mod file_serve;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
