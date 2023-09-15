use std::thread;

fn main() {
    let numbers = vec![1, 2, 3];

    // 传递闭包捕获并移动值到新的线程
    thread::spawn(move || {
        for n in &numbers {
            println!("{n}");
        }
    })
    .join()
    .unwrap();

    // borrow of moved value: `numbers` value borrowed here after move
    // numbers 的所有权被转移到新产生的线程
    // println!("{numbers:?}");
}
