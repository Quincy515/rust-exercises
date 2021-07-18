use chrono::prelude::*;

fn main() {
    let months = [
        "一月",
        "二月",
        "三月",
        "四月",
        "五月",
        "六月",
        "七月",
        "八月",
        "九月",
        "十月",
        "十一月",
        "十二月",
    ];
    let cur_month = Local::today().month();
    let index = cur_month as usize - 1;
    println!("现在是{}", months[index]);

    //    for i in 0..months.len() {
    //        println!("months:{}", months[i]);
    //    }

    for i in months.iter() {
        //      println!("months:{}", i);
        say_hello(i);
    }

    println!("20 + 10 = {}", plus_ten(20));
}

fn say_hello(name: &str) {
    println!("Hello, {}", name);
}

fn plus_ten(num: i32) -> i32 {
    //    return num + 10;
    num + 10
}
