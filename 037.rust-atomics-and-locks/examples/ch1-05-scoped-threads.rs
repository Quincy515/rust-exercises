use std::thread;

fn main() {
    let numbers = vec![1, 2, 3];

    // std::thread::scope 产生作用域内的线程
    // 可以安全地借用局部变量
    thread::scope(|s| {
        s.spawn(|| {
            println!("length: {}", numbers.len());
        });
        s.spawn(|| {
            for n in &numbers {
                println!("{n}");
            }
        });
    });

    println!("{numbers:?}");
}
