// alert('xxx')
// use wasm_bindgen::prelude::*;
// #[wasm_bindgen]
// extern "C" {
//     pub fn alert(s: &str);
// }

use web_sys::console;
use web_sys::window;

// window.alert
pub fn alert(str: &str) {
    window().unwrap().alert_with_message(str).unwrap();
}

// console.log(xxx)
pub fn log(obj: &wasm_bindgen::JsValue) {
    console::log_1(obj);
}
