# [1083.CSES - Missing Number](https://cses.fi/problemset/task/1083/)

You are given all numbers between $1,2,…,n$ except one. Your task is to find the missing number.

**Input**

The first input line contains an integer $n$.

The second line contains $n−1$ numbers. Each number is distinct and between $1$ and $n$ (inclusive).

**Output**

Print the missing number.

**Constraints**

- $ 2 \le n \le 2\times10^5$

**Example**

Input:
`5`

`2 3 1 5`

Output:
`4`

**思路**

常见数据结构使用，数组，hashmap 使用，数学等差数列求和

**题解**

```rust
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

fn main() {
    // read first line
    let mut n: String = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n: i64 = n.trim().parse().unwrap();
    // read second line
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let vec: Vec<i64> = line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut hash: HashMap<i64, bool> = HashMap::new();
    vec.iter().for_each(|v| {
        hash.insert(*v, true);
    });
    for i in 1..n {
        if !hash.contains_key(&i) {
            println!("{}", i);
            return;
        }
    }
    println!("{}", n);
}
```

