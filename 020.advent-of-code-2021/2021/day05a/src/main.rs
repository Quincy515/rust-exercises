use std::fs;

fn main() {
    let content = fs::read_to_string("./day05a/test.txt").unwrap();
    println!("{:?}", content);
}

