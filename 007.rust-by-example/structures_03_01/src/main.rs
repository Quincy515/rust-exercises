#[derive(Debug)]
struct Person<'a> {
  name: &'a str,
  age: u8,
}

// 单元结构体
struct Nil;

// 元组结构体
struct Pair(i32, f32);

// 带有两个字段(field)的结构体
struct Point{
  x: f32,
  y: f32,
}

// 结构体可以作为另一个结构体的字段
#[allow(dead_code)]
struct Rectangle {
  top_left: Point,
  bottom_right: Point,
}

impl Rectangle {
  // 计算长方形面积的函数 rect_area（使用嵌套的解构方式）
  fn rect_area(&self) -> f32 {
    let Rectangle { top_left: top, bottom_right: bottom} =  self;
    (top.y - bottom.y) * (bottom.x - top.x)
  }
}

// 函数 square，接受的参数是一个 Point 和一个 f32，并返回一个 Rectangle（长方形），
// 其左下角的点等于 Point 参数，长和宽都等于 f32 参数。
fn square(point: Point, nbr: f32) -> Rectangle {
  Rectangle {
    top_left: Point { x: point.x, y: point.y + nbr },
    bottom_right: Point { x: point.x + nbr, y: point.y}
  }
}

fn main() {
  // 使用简单的写法初始化字段，并创建结构体
  let name = "Peter";
  let age = 27;
  let peter = Person {name, age};

  // 以 Debug 方式打印结构体
  println!("{:?}", peter);

  // 实例化结构体 `Point`
  let point: Point = Point {x: 10.3, y: 0.4};
  // 访问 point 字段
  println!("point coordinates: ({}, {})", point.x, point.y);

  // 使用结构体更新语法创建新的 point，这样可以用到之前的 point 字段
  let bottom_right = Point {x:5.2, ..point};
  // `bottom_right.y ` 与 `point.y` 一样，因为这个字段就是从 `point` 中来的
  println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

  // 使用 `let` 绑定来解构 point
  let Point {x: top_edge , y: left_edge } = point;
  let _rectangle = Rectangle {
      // 结构体的实例化也是一个表达式
      top_left: Point{x:left_edge, y:top_edge},
      bottom_right
  };

  // 实例化一个单元结构体
  let _nil = Nil;

  // 实例化一个元组结构体
  let pair = Pair(1, 0.1);

  // 访问元组结构体的字段
  println!("pair contains {:?} and {:?}", pair.0, pair.1);

  // 解构一个元组结构体
  let Pair(integer, decimal) = pair;
  println!("pair contains {:?} and {:?}", integer, decimal);

  println!("area of rectangle is {:?}", _rectangle.rect_area());
  let rect = square(Point{x: 10.2, y: 5.0}, 10.2);
  println!("area of new rectangle is {:?}", rect.rect_area());
}