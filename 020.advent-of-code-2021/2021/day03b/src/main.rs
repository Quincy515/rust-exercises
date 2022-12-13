use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("./day03b/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut content_vec = vec![];
    for line in contents.lines() {
        content_vec.push(line.to_string());
    }
    let count = content_vec[0].len();

    let oxygen = fun_name(count, '0', '1', content_vec.to_owned());
    println!("oxygen: {:?}", oxygen);

    let co2 = fun_name(count, '1', '0', content_vec.to_owned());
    println!("co2: {:?}", co2);

    // 计算
    println!("{}", get_value(oxygen) * get_value(co2));
}

fn get_value(value: String) -> i32 {
    let mut rate = 0;
    let mut index: i32 = -1;
    for i in value.chars().rev() {
        index += 1;
        if i == '1' {
            rate += 2_i32.pow(index as u32);
        }
    }
    rate
}

fn fun_name(count: usize, find1: char, find2: char, mut vector: Vec<String>) -> String {
    for i in 0..count {
        if vector.len() == 1 {
            return vector[0].to_string();
        }
        // 1. 计算每行数字的每一位总和
        let sum = get_sum_at_index(i, &vector);
        // println!("当i: {}, sum: {}, total: {}", i, sum, vector.len());

        // 2.
        if vector.len() > sum * 2 {
            vector = find_remain_num(i, find1, &vector);
        } else {
            vector = find_remain_num(i, find2, &vector);
        }
        // println!("vector: {:?}", vector);
    }
    vector[0].to_string()
}

fn find_remain_num(i: usize, eq: char, vec: &Vec<String>) -> Vec<String> {
    let mut ret = vec![];
    for string in vec {
        if string.chars().nth(i).unwrap() == eq {
            ret.push(string.to_owned());
        }
    }
    ret
}

fn get_sum_at_index(index: usize, contents: &Vec<String>) -> usize {
    let mut sum = 0;
    for line in contents {
        for (i, char) in line.chars().enumerate() {
            if i == index {
                sum += char.to_string().parse::<i32>().unwrap();
            }
        }
    }
    sum as usize
}
