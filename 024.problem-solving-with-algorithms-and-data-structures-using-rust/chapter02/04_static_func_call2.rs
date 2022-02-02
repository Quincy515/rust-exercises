use std::time::SystemTime;

fn sum_of_n2(n: i64) -> i64 {
    n * (n + 1) / 2
}

fn main() {
    for _i in 0..5 {
        let now = SystemTime::now();
        let _sum = sum_of_n2(1000000);
        let duration = now.elapsed().unwrap();
        println!("func used {} ns", duration.as_nanos());
    }
}
