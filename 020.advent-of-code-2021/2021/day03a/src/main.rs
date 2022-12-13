use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("./day03a/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut count = 0; // 记录每一行有几位数字
    for line in contents.lines() {
        for _ in line.chars() {
            count += 1;            
        }
        break;
    }

    let mut total = 0; // 记录总行数
    let mut vector =vec![0; count]; // 记录每个数字出现的次数
    for line in contents.lines() {
        total += 1;
        for (i, char) in line.chars().enumerate() {
            vector[i] += char.to_string().parse::<i32>().unwrap();
        }
    }
    println!("total: {}. vector: {:?}", total, vector);

    let mut gamma = String::new();
    let mut epsilon = String::new();
    for i in vector {
        if total / 2 > i {
            gamma.push_str("0");
            epsilon.push_str("1");
        } else {
            gamma.push_str("1");
            epsilon.push_str("0");
        }
    }
    println!("gamma: {}, epsilon: {}", gamma, epsilon);

    let mut gamma_rate = 0;
    let mut index: i32 = -1;
    for i in gamma.chars().rev() {
        index += 1;
        if i == '1' {
            gamma_rate += 2_i32.pow(index as u32);
        }
    }

    let mut epsilon_rate = 0;
    let mut index: i32 = -1;
    for i in epsilon.chars().rev() {
        index += 1;
        if i == '1' {
            epsilon_rate += 2_i32.pow(index as u32);
        }
    }
    println!("gamma_rate: {}, epsilon_rate: {}", gamma_rate, epsilon_rate);
    println!("total: {}", gamma_rate * epsilon_rate);
}
