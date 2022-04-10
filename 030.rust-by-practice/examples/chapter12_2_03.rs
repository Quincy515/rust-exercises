/// 3. 🌟🌟🌟 当执行错误处理时，
/// 为我们自定义的错误类型实现 From 特征是非常有用。
/// 这样就可以通过 ? 自动将某个错误类型转换成我们自定义的错误类型



use std::fs;
use std::io;
use std::num;

enum CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for CliError {
    // 实现 from 方法
    fn from(err: io::Error) -> Self {
        CliError::IoError(err)
    }
}

impl From<num::ParseIntError> for CliError {
    // 实现 from 方法
    fn from(err: num::ParseIntError) -> Self {
        CliError::ParseError(err)
    }
}

fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
    // ? 自动将 io::Error 转换成 CliError
    let contents = fs::read_to_string(&file_name)?;
    // num::ParseIntError -> CliError
    let num: i32 = contents.trim().parse()?;
    Ok(num)
}

fn main() {
    println!("Success!")
}

