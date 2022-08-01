# [1068.CSES - Weird Algorithm](https://cses.fi/problemset/task/1068)

Consider an algorithm that takes as input a positive integer nn. If nn is even, the algorithm divides it by two, and if nn is odd, the algorithm multiplies it by three and adds one. The algorithm repeats this, until nn is one. For example, the sequence for n=3n=3 is as follows:

$3→10→5→16→8→4→2→13→10→5→16→8→4→2→1$


Your task is to simulate the execution of the algorithm for a given value of nn.

**Input**

The only input line contains an integer nn.

**Output**

Print a line that contains all values of nn during the algorithm.

**Constraints**

- $1 \le n \le 10^6$

**Example**

Input:
`3`

Output:
`3 10 5 16 8 4 2 1`

**思路**

熟悉常见输入输出

**题解**

```rust
use std::io::{BufRead, BufReader};
fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut num: u64 = split.next().unwrap().parse().unwrap();
    print!("{}", num);
 
    while num != 1 {
        if num % 2 == 0 {
            num /= 2;
        } else {
            num = (num * 3) + 1;
        }
 
        print!(" {}", num);
    }
}
```

