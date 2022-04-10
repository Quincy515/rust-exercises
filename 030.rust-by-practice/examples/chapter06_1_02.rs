//2.🌟🌟 如果要使用 str 类型，只能配合 Box。 & 可以用来将 Box<str> 转换为 &str 类型

// 使用至少两种方法来修复错误
fn main() {
    let s: Box<str> =  "hello, world".into();
    greetings(&s);

    let s2: Box<&str> =  "hello, world".into();
    greetings(*s2);
}

fn greetings(s: &str) {
    println!("{}",s)
}

