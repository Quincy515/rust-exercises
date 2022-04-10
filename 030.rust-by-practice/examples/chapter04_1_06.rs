///
/// // 修改 `assert!` 让代码工作
///fn main() {
///   let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
///    assert!(v == 1579);
///}

// 修改 `assert!` 让代码工作
fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    println!("{v}");
    assert!(v == 1597);
}
