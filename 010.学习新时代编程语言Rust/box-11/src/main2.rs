use std::mem;

#[derive(Debug)]
struct Point {
    x: u64,
    y: u64
}

#[derive(Debug)]
struct Point3D {
    x: u64,
    y: u64,
    z: u64
}

fn origin() -> Point {
    Point{x:0, y:0}
}

fn origin_3d() -> Point3D {
    Point3D{x: 0, y:0, z:0}
}

fn main() {
    let mut point = origin();
    println!("point: {:?}", &point);
    println!("point size: {}", mem::size_of_val(&point));

    let point_3d = origin_3d();
    println!("3d point size: {:?}",mem::size_of_val(&point_3d));

    let mut boxed_point = Box::new(origin());
    println!("boxed point size: {}", mem::size_of_val(&boxed_point));

    let boxed_point_3d = Box::new(origin_3d());
    println!("3d boxed point size: {:?}",mem::size_of_val(&boxed_point_3d));
}
