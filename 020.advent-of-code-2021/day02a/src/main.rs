use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("./day02a/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut x = 0;
    let mut y = 0;
    for line in contents.lines() {
        let mut param = line.split_whitespace();
        match(param.next(), param.next()) {
            (Some(order), Some(value)) => {
                if order == "forward" {
                    x += value.parse::<i32>().unwrap();
                } else if order == "down" {
                    y += value.parse::<i32>().unwrap();
                } else if order == "up" {
                    y -= value.parse::<i32>().unwrap();
                }
            },
            _ => {
                println!("Error parsing input");
            }
        }
    }
    println!("{}", x * y);    
}
