use std::{fs::File, io::Read};

fn main() {
    // 1. 读取文件
    let mut file = File::open("./day01b/input.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    // 2. 把文件内容转成数组
    let inputs = content
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    // 3. 遍历数组计算滑动窗口3个数之和
    let mut windows = vec![];
    for i in 2..inputs.len() {
        windows.push(inputs[i] + inputs[i - 1] + inputs[i - 2]);
    }
    // 4. 遍历窗口数组，计算后一个数比前一个数大的数量
    let mut count = 0;
    for i in 1..windows.len() {
        if windows[i] > windows[i - 1] {
            count += 1;
        }
    }
    println!("Answer: {:?}", count);
}
