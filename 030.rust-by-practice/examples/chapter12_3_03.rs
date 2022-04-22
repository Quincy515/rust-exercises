use std::num::ParseIntError;
/// 3. 🌟🌟 还可以为自定义类型实现 FromStr 特征
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s
            .trim_matches(|p| p == '(' || p == ')')
            .split(',')
            .collect();

        let x_fromstr = coords[0].parse::<i32>()?;
        let y_fromstr = coords[1].parse::<i32>()?;

        Ok(Point {
            x: x_fromstr,
            y: y_fromstr,
        })
    }
}
fn main() {
    // 使用两种方式填空
    // 不要修改其它地方的代码
    let p = Point::from_str("(3,4)");
    assert_eq!(p.unwrap(), Point { x: 3, y: 4 });
    let p ="(3,4)".parse::<Point>(); 
    assert_eq!(p.unwrap(), Point { x: 3, y: 4 });

    println!("Success!")
}
