/// 2. 🌟🌟 默认情况下, 数值溢出会导致编译错误，
/// 但是我们可以通过添加一行全局注解的方式来避免编译错误(溢出还是会发生)
#[deny(overflowing_literals)]
fn main() {
    assert_eq!(u8::MAX, 255);
    // 如上所示，u8 类型允许的最大值是 255.
    // 因此以下代码会报溢出的错误： literal out of range for `u8`.
    // **请仔细查看相应的编译错误，从中寻找到解决的办法**
    // **不要修改 main 中的任何代码**
    let v = 1000 as u8;

    println!("Success!")
}

