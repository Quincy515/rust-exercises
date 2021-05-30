fn main() {
    let x: Box<i32> = Box::new(5);
    let y: Box<i32> = x; // 产生所有权的转移。

    // 再次调用变量 x，由于 x 已被释放，会抛出 "borrow of moved value: `x`" 的错误。
    println!("x: {}", x);
    println!("y: {}", y);
}
