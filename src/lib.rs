pub mod app;
pub mod components;
pub mod content;
pub mod pages;

use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "hydrate")] {
    use leptos::*;
    use wasm_bindgen::prelude::wasm_bindgen;
    use crate::app::*;

    #[wasm_bindgen]
    pub fn hydrate() {
        leptos::mount_to_body(move |cx| {
            view! { cx, <App/> }
        });
    }
}}
