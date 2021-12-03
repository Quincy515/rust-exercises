use std::{fs::File, io::Read};

fn main() {
    // 1. 读取文件
    let mut file = File::open("./day01a/input.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    // 2. 把文件内容转成数组
    let inputs = content
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    // 3. 遍历数组计算后一个数比前一个数大的个数
    let mut count = 0;
    for i in 1..inputs.len() {
        if inputs[i] > inputs[i - 1] {
            count += 1;
        }
    }
    println!("Answer: {:?}", count);
}
