fn main() {
    let x: Box<i32> = Box::new(5);
    let y: i32 = *x; // 通过解引用操作符获取变量 x 所指向的 i32 类型的值。将改值按位复制给变量 y，再次调用 x 就不会报错

    println!("x: {}", x);
    println!("y: {}", y);
}
