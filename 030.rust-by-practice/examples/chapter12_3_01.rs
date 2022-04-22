/// 其它转换
/// 将任何类型转换成 String
/// 只要为一个类型实现了 ToString，
/// 就可以将任何类型转换成 String。
/// 事实上，这种方式并不是最好的，
/// 大家还记得 fmt::Display 特征吗？
/// 它可以控制一个类型如何打印，在实现它的时候还会自动实现 ToString。

/// 1. 🌟🌟


use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    // 实现 fmt 方法
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The point is ({}, {})", self.x, self.y)
    }
}

fn main() {
    let origin = Point { x: 0, y: 0 };
    // 填空
    assert_eq!(origin.to_string(), "The point is (0, 0)");
    assert_eq!(format!("{origin}"), "The point is (0, 0)");

    println!("Success!")
}
