// In example projects but not explained
// Keeping just in case
#![recursion_limit = "1024"]

mod app;
mod data;
mod search;

use wasm_bindgen::prelude::*;

// Entry
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    set_panic_hook();
    web_logger::init();
    yew::start_app::<app::App>();
    Ok(())
}

fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
