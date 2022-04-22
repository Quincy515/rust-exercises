/// Exercises
/// Method
/// 1. 🌟🌟 方法跟函数类似：都是使用 fn 声明，有参数和返回值。
/// 但是与函数不同的是，方法定义在结构体的上下文中(枚举、特征对象也可以定义方法)，
/// 而且方法的第一个参数一定是 self 或其变体 &self 、
/// &mut self，self 代表了当前调用的结构体实例。

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 完成 area 方法，返回矩形 Rectangle 的面积
    fn area(self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    assert_eq!(rect1.area(), 1500);
}

