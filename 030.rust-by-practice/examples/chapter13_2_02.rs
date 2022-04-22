/// ?
/// ? 跟 unwrap 非常像，但是 ? 会返回一个错误，而不是直接 panic.
/// 2. 🌟🌟


use std::num::ParseIntError;

// 使用 `?` 来实现 multiply
// 不要使用 unwrap !
fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    let n1 = n1_str.parse::<i32>()?;
    let n2 = n2_str.parse::<i32>()?;
    Ok(n1 * n2)
}

fn main() {
    assert_eq!(multiply("3", "4").unwrap(), 12);
    println!("Success!")
}
