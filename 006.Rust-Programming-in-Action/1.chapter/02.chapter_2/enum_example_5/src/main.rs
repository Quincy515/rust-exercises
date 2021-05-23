#[derive(Debug)]
enum ColorNoParam {
    Red, // 枚举值
    Yellow,
    Blue,
} // 枚举值不带有类型参数，称为无参数枚举类型

#[derive(Debug)]
enum ColorParam {
    Red(String),    // 枚举类型带有 String 类型参数，类似于函数调用
    Yellow(String), // 使用这种枚举值需要传入实参
    Blue(String),
} // 称为带参数枚举类型

fn main() {
    let color_no_param = ColorNoParam::Red;
    match color_no_param {
        ColorNoParam::Red => println!("{:?}", ColorNoParam::Red),
        ColorNoParam::Yellow => println!("{:?}", ColorNoParam::Yellow),
        ColorNoParam::Blue => println!("{:?}", ColorNoParam::Blue),
    }

    println!("{:?}", ColorParam::Blue(String::from("blue")));
}
