/// 结构体上的一些操作
/// 4. 🌟 你可以在实例化一个结构体时将它整体标记为可变的，
/// 但是 Rust 不允许我们将结构体的某个字段专门指定为可变的.


// 填空并修复错误，不要增加或移除代码行
struct Person {
    name: String,
    age: u8,
}
fn main() {
    let age = 18;
    let mut p = Person {
        name: String::from("sunface"),
        age,
    };

    // how can you believe sunface is only 18? 
    p.age = 30;

    // 填空
    p.name = String::from("sunfei");
}

