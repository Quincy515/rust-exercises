/// 结构体和 impl
/// 3. 🌟


// 实现一个结构体 Point 让代码工作

struct Point<T> {
    x: T,
    y: T
}
fn main() {
    let _integer = Point { x: 5, y: 10 };
    let _float = Point { x: 1.0, y: 4.0 };
}

