use std::thread;

fn main() {
    let mut numbers = vec![1, 2, 3];

    thread::scope(|s| {
        s.spawn(|| {
            numbers.push(1);
        });
        // second mutable borrow occurs here
        // s.spawn(|| {
        //     numbers.push(2);
        // });
    });
}
