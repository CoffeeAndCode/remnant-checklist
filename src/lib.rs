#![deny(clippy::all, clippy::cargo, clippy::nursery, clippy::pedantic)]
#![allow(clippy::non_ascii_literal)]
#![recursion_limit = "512"]

mod app;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// # Errors
///
/// Will return `JsValue` if dom selector is unavailable.
#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    yew::initialize();

    let document = yew::utils::document();
    let element = document.query_selector("#app").unwrap().unwrap();
    yew::App::<app::App>::new().mount(element);

    Ok(())
}

#[cfg(test)]
#[allow(clippy::assertions_on_constants)]
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
