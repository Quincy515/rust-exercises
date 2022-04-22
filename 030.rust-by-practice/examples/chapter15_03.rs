/// 3. 🌟🌟 有时我们会期望示例的结果是一个 panic。
/// 将以下代码添加到 src/compute.rs ，并且让 cargo test 成功运行.
/// 你只能修改注释，不要修改 fn div
#![allow(unused)]
fn main() {
// in src/compute.rs

/// # Panics
///
/// The function panics if the second argument is zero.
///
/// ```rust,should_panic
/// // panics on division by zero
/// doc_comments::compute::div(10, 1);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }

    a / b
}
}

