/// 枚举 Enum
/// 1. 🌟🌟 在创建枚举时，你可以使用显式的整数设定枚举成员的值。

#[allow(dead_code)]
// 修复错误
enum Number {
    Zero,
    One,
    Two,
}

enum Number1 {
    Zero = 0isize,
    One,
    Two,
}

// C 语言风格的枚举定义
enum Number2 {
    Zero = 0isize,
    One = 1isize,
    Two = 2isize,
}


fn main() {
    // 通过 `as` 可以将枚举值强转为整数类型
    assert_eq!(Number::One as i32, Number1::One as i32);
    assert_eq!(Number1::One as i32, Number2::One as i32);
} 

