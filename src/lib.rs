mod utils;

use wasm_bindgen::prelude::*;

extern crate wee_alloc;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
// TODO: https://rustwasm.github.io/docs/book/game-of-life/hello-world.html [GUIDE] 
// #[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(s: &str) {
    alert(format!("Hello, {}!", s).as_str());
}
