// trait 本质是一组方法原型，是实现某些目的的行为集合。
// trait 可以包含两种形式的方法：抽象方法 - 没有具体实现的方法，具体方法-带有具体实现的方法

// 求周长和面积是几何图形的共同需求，
// 可以将求周长和面积通过 trait 定义为几何图形共享的行为。
use std::fmt::{Display, Formatter, Result};

trait Geometry {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
}

// trait 作为参数，使用 impl Trait 语法表示参数类型
// 这样可以在函数体内调用 trait 定义的方法，还可以将其用于一些复杂的开发场景
// print 函数的参数类型是 impl Geometry，该参数支持任何实现了 Geometry trait 的结构体实例
fn print(geometry: impl Geometry + Display) {
    println!(
        "{}, area: {}, perimeter: {}",
        geometry,
        geometry.area(),
        geometry.perimeter()
    );
}

// trait 约束作为函数参数
fn print_bound<T: Geometry + Display>(geometry: T) {
    println!(
        "{}, area: {}, perimeter: {}",
        geometry,
        geometry.area(),
        geometry.perimeter()
    );
}

#[derive(Clone, Copy)]
struct Rectangle {
    width: f32,
    height: f32,
}

impl Geometry for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }

    fn perimeter(&self) -> f32 {
        (self.width + self.height) * 2.0
    }
}

// 为结构体 Rectangle 实现 Display trait
impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Rectangle: ({}, {})", self.width, self.height)
    }
}

#[derive(Clone, Copy)]
struct Circle {
    radius: f32,
}

impl Geometry for Circle {
    fn area(&self) -> f32 {
        3.14 * self.radius * self.radius
    }
    fn perimeter(&self) -> f32 {
        3.14 * 2.0 * self.radius
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Circle: ({})", self.radius)
    }
}

// 多 impl Geometry 作为参数类型
// impl Trait 语句支持为多个参数指定类型
fn area_add(geo1: impl Geometry, geo2: impl Geometry) {
    println!(
        "rect.area: {}, circle.area: {}, total area: {}",
        geo1.area(),
        geo2.area(),
        geo1.area() + geo2.area()
    );
}

// 可以对多个泛型参数使用 trait 约束
// 如果泛型参数有多个 trait 约束，那么拥有多个泛型参数的函数，在函数名和参数列表之间会有很长的 trait 约束信息，使得函数签名可读性变差
// fn area_add_long<T: Geometry + Display + Clone, U: Geometry + Display + Debug>(geo1: T, geo2: U) {}
// 在函数签名后面跟上 where 从句，为每个泛型参数指定 trait 约束，这样函数签名的可读性就提高了
fn area_add_nice<T, U>(geo1: T, geo2: U)
where
    T: Geometry,
    U: Geometry,
{
    println!(
        "rect.area: {}, circle.area: {}, total area: {}",
        geo1.area(),
        geo2.area(),
        geo1.area() + geo2.area()
    );
}

fn area_add_bound<T: Geometry, U: Geometry>(geo1: T, geo2: U) {
    println!(
        "rect.area: {}, circle.area: {}, total area: {}",
        geo1.area(),
        geo2.area(),
        geo1.area() + geo2.area()
    );
}

// 函数的返回值类型可以是返回某个实现了 trait 的类型
// return_geometry 函数的返回值类型是 impl Geometry，函数体中返回了实现 Geometry trait 的Rectangle 类型。需要注意的是，这只适用于返回单一类型的情况，如果返回可能为 Rectangle，也可能为 Circle，将会导致程序错误。
fn return_geometry() -> impl Geometry {
    Rectangle {
        width: 12.5,
        height: 5.5,
    }
}

fn main() {
    let rect = Rectangle {
        width: 8.8,
        height: 2.2,
    };

    println!(
        "rect.area: {}, rect.perimeter: {}",
        rect.area(),
        rect.perimeter()
    ); // rect.area: 19.36, rect.perimeter: 22

    let circle = Circle { radius: 3.0 };
    println!(
        "circle.area: {}, circle.perimeter: {}",
        circle.area(),
        circle.perimeter()
    ); // circle.area: 28.26, circle.perimeter: 18.84

    let rect = Rectangle {
        width: 10.5,
        height: 5.5,
    };
    print(rect); // area: 57.75, perimeter: 32

    let rect = Rectangle {
        width: 10.5,
        height: 5.5,
    };
    let circle = Circle { radius: 3.0 };
    area_add(rect, circle); // rect.area: 57.75, circle.area: 28.26, total area: 86.01

    let circle = Circle { radius: 3.0 };
    print_bound(circle); // Circle: (3), area: 28.26, perimeter: 18.84

    let geometry = return_geometry();
    area_add_nice(geometry, rect);
}
