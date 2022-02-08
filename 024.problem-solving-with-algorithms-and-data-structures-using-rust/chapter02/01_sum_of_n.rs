fn sum_of_n(n: i32) -> i32 {
    let mut sum: i32 = 0;
    for i in 1..=n {
        sum += i;
    }
    sum
}

fn main() {
    println!("{}", sum_of_n(10));
}
