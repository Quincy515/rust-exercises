/// 3. 🌟🌟
///
// 问题:  我们的代码中发生了多少次堆内存分配？
// 你的回答: 2 次
fn main() {
    // 基于 `&str` 类型创建一个 String,
    // 字符串字面量的类型是 `&str`
    let s: String = String::from("hello, world!"); // 1.

    // 创建一个切片引用指向 String `s`
    let slice: &str = &s; // 2.

    // 基于刚创建的切片来创建一个 String
    let s: String = slice.to_string();

    assert_eq!(s, "hello, world!");

    println!("Success!")
}

