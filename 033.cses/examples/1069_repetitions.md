# [1069.CSES - Repetitions](https://cses.fi/problemset/task/1069)

You are given a DNA sequence: a string consisting of characters A, C, G, and T. Your task is to find the longest repetition in the sequence. This is a maximum-length substring containing only one type of character.

**Input**

The only input line contains a string of nn characters.

**Output**

Print one integer: the length of the longest repetition.

**Constraints**

- $1 \le n \le 10^6$

**Example**

Input:
`ATTCGGGA`

Output:
`3`

**思路**

1、初始化双指针 i，j 分别指向第 0 个字符，和第 1 个字符

2、如果 i、j 指向的字符相等，j 向后移动一位

3、如果 i、j 指向的字符不等，i 移动到 j 的位置，j 向后移动一位

4、这上述过程中记录最大的重复子串长度 `j - i + 1`

**题解**

```rust
fn repetitions(str: Vec<char>) -> i32 {
    let (mut i, mut j) = (0, 1);
    let mut max = 1;
    while i < str.len() && j < str.len() {
        if str[i] == str[j] {
            max = max.max(j - i + 1);
            j += 1;
        } else {
            i = j;
            j += 1;
        }
    }
    max as i32
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let str = input.trim();
    if str.len() == 1 {
        println!("1");
        return;
    }
    let result = repetitions(str.chars().collect::<Vec<char>>());
    println!("{}", result);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            repetitions("ATTCGGGA".to_string().chars().collect::<Vec<char>>()),
            3
        );
    }
}
```

