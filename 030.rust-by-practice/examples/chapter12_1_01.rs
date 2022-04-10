/// 使用 as 进行类型转换
/// Rust 并没有为基本类型提供隐式的类型转换( coercion )，但是我们可以通过 as 来进行显式地转换。

///1.🌟

// 修复错误，填空
// 不要移除任何代码
fn main() {
    let decimal = 97.123_f32;

    let integer: u8 = decimal as u8;

    let c1: char = decimal as u8 as char;
    let c2 = integer as char;

    assert_eq!(integer, 'b' as u8 - 1);

    println!("Success!")
}

