use std::thread;

fn main() {
    let numbers = Vec::from_iter(0..=1000);
    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.iter().sum::<usize>();
        sum / len // 线程闭包（1）返回的值通过 join 方法发送回主线程
    });

    // 从线程中取回一个值，是从闭包中返回值来完成的。
    // 该返回值可以通过 join 方法返回的 Result 中获取
    let average = t.join().unwrap();
    println!("average:{average}");
}
