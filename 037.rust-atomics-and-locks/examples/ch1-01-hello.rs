use std::thread;

fn main() {
    // thread::spawn 产生新线程
    // 接受一个参数：新线程执行的函数。
    // 一旦该函数返回，线程就会停止。
    thread::spawn(f);
    thread::spawn(f);

    thread::sleep(std::time::Duration::from_secs(2));
    println!("Hello from the main thread.");
}

// 产生两个线程，它们都将执行 f 作为它们的执行函数。
// 这两个线程将输出一个信息并且展示它们的线程 id，主线程也将输出它自己的信息。
fn f() {
    println!("Hello from another thread!");

    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}
