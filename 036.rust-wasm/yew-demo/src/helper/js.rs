// alert('xxx')
// use wasm_bindgen::prelude::*;
// #[wasm_bindgen]
// extern "C" {
//     pub fn alert(s: &str);
// }

use web_sys::window;

// window.alert
pub fn alert(str: &str) {
    window().unwrap().alert_with_message(str).unwrap();
}
