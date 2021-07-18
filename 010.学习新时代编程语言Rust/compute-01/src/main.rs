use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let number1 = args[1].parse::<f32>().unwrap();
    let operator = args[2].parse::<char>().unwrap();
    let number2 = args[3].parse::<f32>().unwrap();
    if operator == '+' {
        println!("{}{}{}={}", number1, operator, number2, number1 + number2);
    } else if operator == '-' {
        println!("{}{}{}={}", number1, operator, number2, number1 - number2);
    } else if operator == '*' {
        println!("{}{}{}={}", number1, operator, number2, number1 * number2);
    } else if operator == '/' {
        println!("{}{}{}={}", number1, operator, number2, number1 / number2);
    } else {
        println!("当前程序只支持加减乘除运算");
    }
}
