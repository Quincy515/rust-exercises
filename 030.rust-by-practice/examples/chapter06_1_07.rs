/// &str 和 String
/// 与 str 的很少使用相比，&str 和 String 类型却非常常用，因此也非常重要。
/// 7. 🌟🌟 我们可以使用两种方法将 &str 转换成 String 类型



// 使用至少两种方法来修复错误
fn main() {
    let s =  "hello, world";
    greetings(s.to_string());


    let s = String::from("hello, world");
    greetings(s);
}

fn greetings(s: String) {
    println!("{}",s)
}

