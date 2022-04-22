fn main() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("x 的值是 {}, y 的值是 {}", x, y);
    }
    let y: i32 = 0;
    println!("x 的值是 {}, y 的值是 {}", x, y);
}
