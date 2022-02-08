fn nums_sum(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    for num in nums {
        sum += num;
    }
    sum
}

fn main() {
    let sum = nums_sum([1, 2, 3, 4].to_vec());
    println!("{}", sum);
}
