/// 在 fn main 中使用 Result
/// 一个典型的 main 函数长这样:


/// fn main() {
///     println!("Hello World!");
/// }
///
/// 事实上 main 函数还可以返回一个 Result 类型：
/// 如果 main 函数内部发生了错误，那该错误会被返回并且打印出一条错误的 debug 信息。




use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(number)  => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);
    Ok(())
}

