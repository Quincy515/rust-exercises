/// 计数和比较法 O(n)
/// 通过分析可知，s1 和 s2 只含 26 个小写字母，
/// 所以用两个长度为 26 的列表，统计各个字符出现的频次就可以了。
/// 每遇到一个字符，就增加该字符在列表对应位置的计数。
/// 最后如果两个列表计数一样，则字符串为乱序字符串。
fn anagram_solution4(s1: &str, s2: &str) -> bool {
    // 字符串长度不同，一定不是乱序字符串
    if s1.len() != s2.len() {
        return false;
    }

    // 大小为 26 的集合，用于将字符映射为 ASCII 值
    let mut c1 = [0; 26];
    let mut c2 = [0; 26];
    for c in s1.chars() {
        let pos = (c as usize) - 97; // 97 为 a 的 ASCII 值
        c1[pos] += 1;
    }
    for c in s2.chars() {
        let pos = (c as usize) - 97; // 97 为 a 的 ASCII 值
        c2[pos] += 1;
    }

    // 逐个比较 ascii 值
    let mut pos = 0;
    let mut ok = true;
    while pos < 26 && ok {
        if c1[pos] == c2[pos] {
            pos += 1;
        } else {
            ok = false;
        }
    }
    ok
}

fn main() {
    let s1 = "rust";
    let s2 = "trus";

    let result: bool = anagram_solution4(&s1, &s2);
    println!("s1 and s2 is anagram: {result}");
}
