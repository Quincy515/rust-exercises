# [[USACO05MAR]Yogurt factory 机器工厂](https://www.luogu.com.cn/problem/P1376)

## 题目描述

小 T 开办了一家机器工厂，在 $N$个星期内，原材料成本和劳动力价格不断起伏，第 $i$ 周生产一台机器需要花费 $C_i$ 元。若没把机器卖出去，每保养一台机器，每周需要花费 $S$ 元，这个费用不会发生变化。

机器工厂接到订单，在第 $i$ 周需要交付 $Y_i$ 台机器给委托人，第 $i$ 周刚生产的机器，或者之前的存货，都可以进行交付。

请你计算出这 $n$ 周时间内完成订单的最小代价。

## 输入格式

第一行输入两个整数 $N$ 和 $S$，接下来 $N$ 行每行两个数 $C_i$ 和 $Y_i$。

## 输出格式

输出一个整数，表示最少的代价。

## 样例 #1

### 样例输入 #1

```
4 5
88 200
89 400
97 300
91 500
```

### 样例输出 #1

```
126900
```

## 提示

$1\leq n\leq 10^4$，$1 \le C_i \le 5000$，$1 \le S\le 100$，$0\le Y_i\le 10^4$。

## Rust
```rust
fn read_line() -> (i32, i32) {
    use std::io;
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let vec: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    (vec[0], vec[1])
}
fn main() {
    let (n, s) = read_line();

    let mut min_cost = i32::MAX - s;
    let mut total = 0i64;
    for _ in 0..n {
        let (c, y) = read_line();
        min_cost = c.min(min_cost + s);
        total += min_cost as i64 * y as i64;
    }
    println!("{}", total);
}
```