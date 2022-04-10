/// 3. 🌟 过长的元组无法被打印输出


// 修复代码错误
fn main() {
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,/* 13*/);
    println!("too long tuple: {:?}", too_long_tuple);
}

