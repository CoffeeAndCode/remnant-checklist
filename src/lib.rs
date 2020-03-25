#![recursion_limit = "512"]

mod app;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    yew::start_app::<app::App>();

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
