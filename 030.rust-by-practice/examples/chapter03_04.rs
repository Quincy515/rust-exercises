fn main() {
    println!("{}, world", define_x()); 
}

fn define_x() -> String {
    let x = "hello".to_string();
    x
}
