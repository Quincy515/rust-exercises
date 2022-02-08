// 标量类型代表一个单独的值，复合类型是标量类型的组合
fn main() {
    // 有四种基本的标量类型：整型、浮点型、布尔型、字符型；
    let a: i8 = -2;
    let b: f32 = 2.35;
    let c: bool = true;
    let d: char = 'a';

    // 两种复合类型：元组、数组
    let x: (i32, f64, u8) = (200, 5.32, 1);
    let xi32 = x.0;
    let yf64 = x.1;
    let xu8 = x.2;
}
