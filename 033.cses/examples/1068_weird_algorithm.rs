use std::io::{BufRead, BufReader};
fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut num: u64 = split.next().unwrap().parse().unwrap();
    print!("{}", num);

    while num != 1 {
        if num % 2 == 0 {
            num /= 2;
        } else {
            num = (num * 3) + 1;
        }

        print!(" {}", num);
    }
}
