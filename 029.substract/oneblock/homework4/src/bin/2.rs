/// 实现一个函数，为u32类型的整数集合求和，参数类型为 &[u32]，返回类型为Option，溢出时返回None
fn main() {
    let v:Vec<u8> = vec![9, 251];
    let sum = sum(&v);
    println!("sum is: {:?}", sum);

    let v: Vec<u32> = vec![1,2, 3];
    let sum = sum_u32(&v);
    println!("sum is: {:?}", sum);
}

fn sum_u32(v: &[u32]) -> Option<u32> {
    v.iter().try_fold(0u32, |acc, &x| acc.checked_add(x))
}

fn sum(v: &[u8]) -> Option<i32> {
    let mut sum:i32 = 0;
    for i in v {
        sum += *i as i32;
    }
    if sum > 255 {
        None
    } else {
        Some(sum)
    }
}