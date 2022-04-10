///解析 String
/// 2. 🌟🌟🌟 使用 parse 方法可以将一个 String 转换成 i32 数字，
/// 这是因为在标准库中为 i32 类型实现了 FromStr: : impl FromStr for i32 


// 为了使用 `from_str` 方法, 你需要引入该特征到当前作用域中
use std::str::FromStr;
fn main() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let from_str = i32::from_str("20").unwrap();
    let sum = parsed + turbo_parsed + from_str;
    assert_eq!(sum, 35);

    println!("Success!")
}
