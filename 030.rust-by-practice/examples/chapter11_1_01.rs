/// String
/// std::string::String 是 UTF-8 编码、可增长的动态字符串.
/// 它也是我们日常开发中最常用的字符串类型，同时对于它所拥有的内容拥有所有权。

/// 基本操作
/// 1. 🌟🌟


// 填空并修复错误
// 1. 不要使用 `to_string()`
// 2. 不要添加/删除任何代码行
fn main() {
    let mut s: String = String::from("hello, ");
    s.push_str("world");
    s.push('!');

    let s = move_ownership(s);

    assert_eq!(s, "hello, world!");

    move_ownership_clone(s.clone());
    assert_eq!(s, "hello, world!");

    borrow_string(&s);
    assert_eq!(s, "hello, world!");
    
    println!("Success!")
}

fn move_ownership(s: String) -> String {
    println!("ownership of \"{}\" is moved here!", s);
    s
}

fn move_ownership_clone(s: String) {
    println!("ownership of \"{}\" is moved here!", s)
}

fn borrow_string(s: &str) {
    println!("ownership of \"{}\" is moved here!", s)
}


