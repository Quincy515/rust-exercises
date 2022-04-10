/// 实现一个打印图形面积的函数，
/// 它接收一个可以计算面积的类型作为参数，
/// 比如圆形，三角形，正方形，需要用到泛型和泛型约束
enum A {
    Cicle(f32),
    Triangle { a: f32, b: f32, c: f32 },
    Square { x: f32, y: f32 },
}

trait Area {
    fn area(&self) -> f32;
}

impl Area for A {
    fn area(&self) -> f32 {
        match self {
            Self::Cicle(r) => std::f32::consts::PI * r * r,
            Self::Triangle { a, b, c } => {
                let s = (a + b + c) / 2.0;
                let area = s * (s - a) * (s - b) * (s - c);
                area.sqrt()
            }
            Self::Square { x, y } => x * y,
        }
    }
}

fn main() {
    dbg!(A::Cicle(10.0).area());
    dbg!(A::Triangle {
        a: 3.0,
        b: 4.0,
        c: 5.0
    }
    .area());
    dbg!(A::Square { x: 10.0, y: 20.0 }.area());
}
