fn main() {
    let x: i32 = 5;
    let y: Box<i32> = Box::new(x);

    assert_eq!(5, *y);
    println!("pointer: {:p}", y);
}
