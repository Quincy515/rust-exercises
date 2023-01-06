use std::thread;

fn main() {
    thread::spawn(f);
    thread::spawn(f);

    thread::sleep(std::time::Duration::from_secs(2));
    println!("Hello from the main thread.");
}

fn f() {
    println!("Hello from another thread!");

    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}