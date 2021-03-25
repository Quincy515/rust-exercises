use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数!");
    let secret_number = rand::thread_rng().gen_range(1..101); // i32 u32 i64
    loop {
        println!("猜测一个数");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");
        // shadow
        // let guess: u32 = guess.trim().parse().expect("请输入一个数字");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if guess < 1 || guess > 100 {
            println!("The guess number will be between 1 and 100.");
            continue;
        }
        println!("你猜测的数是: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), // arm
            Ordering::Greater => println!("To big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
