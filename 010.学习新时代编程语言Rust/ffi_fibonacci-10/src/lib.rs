use wasm_bindgen::prelude::*;

#[no_mangle]
#[wasm_bindgen]
pub fn fibonacci(n: u64) -> u64 {
    if n <= 2 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
#[cfg(test)]
mod tests {}
