/// 5. 🌟 使用结构体字段初始化缩略语法可以减少一些重复代码


// 填空
struct Person {
    name: String,
    age: u8,
}
fn main() {} 

fn build_person(name: String, age: u8) -> Person {
    Person {
        age,
        name,
    }
}

