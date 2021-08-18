use std::mem;

#[derive(Debug)]
struct Point {
    x: u64,
    y: u64
}

fn origin() -> Point {
    Point{x:0, y:0}
}
fn main() {
    let mut point = origin();
    println!("point: {:?}", &point);
    println!("point size: {}", mem::size_of_val(&point));

    let mut boxed_point = Box::new(origin());
    println!("boxed point size: {}", mem::size_of_val(&boxed_point));
}
