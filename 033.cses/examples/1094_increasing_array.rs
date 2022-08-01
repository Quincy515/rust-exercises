fn solved(mut nums: Vec<i64>) -> i64 {
    let mut res = 0;
    for i in 1..nums.len() {
        if nums[i] < nums[i - 1] {
            res += nums[i - 1] - nums[i];
            nums[i] = nums[i - 1];
        }
    }

    res
}

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let vec: Vec<i64> = line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    println!("{}", solved(vec));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solved(vec![3, 2, 5, 1, 7]), 5);
        assert_eq!(
            solved(vec![1000000000, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
            8999999991
        );
    }
}
