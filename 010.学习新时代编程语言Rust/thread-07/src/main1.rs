fn a_print() {
    for i in 1..11 {
        std::thread::sleep(std::time::Duration::from_millis(500));
        println!("a print {}", i);
    }
}
fn b_print() {
    for i in 1..11 {
        std::thread::sleep(std::time::Duration::from_millis(500));
        println!("b print {}", i);
    }
}
fn main() {
    println!("Hello, world!");
    let now = std::time::Instant::now();
    a_print();
    b_print();
    println!("past seconds {}", now.elapsed().as_secs());
}
