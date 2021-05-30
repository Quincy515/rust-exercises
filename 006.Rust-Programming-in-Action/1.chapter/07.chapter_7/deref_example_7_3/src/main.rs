fn main() {
    let x: i32 = 5;
    let y: &i32 = &x; // y 是变量 x 的引用，其类型是 &i32

    assert_eq!(5, *y); // 使用 *y 来访问 y 所指向的 i32 类型的值。
    println!("pointer: {:p}", y); // 这个过程叫做解引用。打印 y 的指针地址，证明其是一个指针
}
