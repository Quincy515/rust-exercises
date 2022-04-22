/// map & and_then
/// map and and_then 是两个常用的组合器( combinator )，
/// 可以用于 Result<T, E> (也可用于 Option<T>).

/// 4. 🌟🌟

use std::num::ParseIntError;

// 使用两种方式填空: map, and then
fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
   n_str.parse::<i32>().and_then(|s| Ok(s+2));
   n_str.parse::<i32>().map(|s| s+2)
}

fn main() {
    assert_eq!(add_two("4").unwrap(), 6);

    println!("Success!")
}

