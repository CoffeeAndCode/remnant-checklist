//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use js_sys::Number;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
async fn my_async_test() {
    // Create an array of promises that are ready on the next tick of the micro task queue.
    let promises = js_sys::Array::new_with_length(3);
    promises.fill(&js_sys::Promise::resolve(&Number::from(42)), 0, 3);
    let promise = js_sys::Promise::all(&promises);

    // Convert that promise into a future and make the test wait on it.
    let x = JsFuture::from(promise).await.unwrap();
    let iter = js_sys::try_iter(&x)
        .unwrap()
        .unwrap()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    assert_eq!(iter, vec![42, 42, 42]);
}
