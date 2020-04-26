#![deny(clippy::all)]
#![recursion_limit = "512"]

mod app;

use wasm_bindgen::prelude::*;
use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    yew::initialize();

    let document = yew::utils::document();
    let element = document.query_selector("#app").unwrap().unwrap();
    yew::App::<app::App>::new().mount(element);

    console::log_1(&"hello there".into());

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(true);
    }

    #[test]
    fn test_example2() {
        assert!(true);
    }
}
