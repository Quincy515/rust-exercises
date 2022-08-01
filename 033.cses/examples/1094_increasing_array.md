# [1094.CSES - Increasing Array](https://cses.fi/problemset/task/1094)

You are given an array of $n$ integers. You want to modify the array so that it is increasing, i.e., every element is at least as large as the previous element.

On each move, you may increase the value of any element by one. What is the minimum number of moves required?

**Input**

The first input line contains an integer $n$: the size of the array.

Then, the second line contains nn integers $x1,x2,…,xn$: the contents of the array.

**Output**

Print the minimum number of moves.

**Constraints**

- $1 \le n \le 2 \times 10^5$
- $1 \le x_i \le 10^9$

**Example**

Input:
`5`

`3 2 5 1 7`

Output:
`5`

**思路**

1、从数组下标 1 的位置，开始遍历数组

2、当前位置的元素小于前一个元素时，记录差值，注意同时需要修改原数组

3、遍历数组结束时，返回记录的差值总和

**题解**

```rust
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
```

