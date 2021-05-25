fn main() {
    let x: u16 = 7;
    let y = x as u32; // 短类型向长类型转换
    println!("u16: {}, u32:{}", x, y);

    let x = std::u32::MAX;
    let y = x as u16; // 长类型转换为短类型会被截断处理
    println!("u32: {}, u16: {}", x, y);

    let x = 65u8;
    let y = x as char; // u8 和 char 相互转换
    println!("u8: {}, char: {}", x, y);

    let x = 'A';
    let y = x as u8;
    println!("char: {}, u8: {}", x, y);

    let x = 7;
    let y = x as f64;
    println!("i32: {}, f64: {}", x, y);

    let x = 7.7;
    let y = x as i32; // f64 类型转换为 i32 类型会有精度丢失问题
    println!("f64: {}, i32: {}", x, y);

    let x = 7;
    let y = x.to_string();
    println!("i32: {}, String: {}", x, y);

    let x = 7.7;
    let y = x.to_string();
    println!("f64: {}, String: {}", x, y);

    let x = String::from("7");
    let y = x.parse::<i32>().unwrap(); // ::<> 叫做 turbofish 操作符，用于为泛型类型指定具体的类型
    println!("String: {}, i32: {}", x, y);

    let x = String::from("7.7");
    let y = x.parse::<f64>().unwrap(); // 返回的是 Result<f64, ParseIntError> 类型，使用 unwrap 方法可获取 Result 中 f64 类型的值。
    println!("String: {}, f64: {}", x, y);

    let x = String::from("hello");
    let y = x.as_str();
    println!("String: {}, &str: {}", x, y);

    let x = "hello";
    let y = x.to_string();
    println!("&str: {}, String: {}", x, y);
}
