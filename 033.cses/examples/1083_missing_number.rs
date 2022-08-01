use std::collections::HashMap;
use std::io::{BufRead, BufReader};

fn main() {
    // read first line
    let mut n: String = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n: i64 = n.trim().parse().unwrap();
    // read second line
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let vec: Vec<i64> = line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut hash: HashMap<i64, bool> = HashMap::new();
    vec.iter().for_each(|v| {
        hash.insert(*v, true);
    });
    for i in 1..n {
        if !hash.contains_key(&i) {
            println!("{}", i);
            return;
        }
    }
    println!("{}", n);
}
