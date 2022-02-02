/// 排序和比较法
/// 按照字母顺序从 a 到 z 排列每个字符串
/// 如果排列后的两个字符串相同，那这两个字符串就是乱序字符串。
/// 其复杂度通常是 O(n^2) 或 O(nlogn)，复杂度跟排序算法属于同样数量级
fn anagram_solution3(s1: &str, s2: &str) -> bool {
    // 字符串长度不同，一定不是乱序字符串
    if s1.len() != s2.len() {
        return false;
    }

    // s1 和 s2 中的字符分别加入 alist, blist
    let mut alist = Vec::new();
    let mut blist = Vec::new();
    for c in s1.chars() {
        alist.push(c);
    }
    for c in s2.chars() {
        blist.push(c);
    }

    // 字符排序
    alist.sort();
    blist.sort();

    // 逐个比较排序的集合，任何字符不匹配就退出循环
    for i in 0..s1.len() {
        if alist[i] != blist[i] {
            return false;
        }
    }

    true
}

fn main() {
    let s1 = "rust";
    let s2 = "trus";

    let result: bool = anagram_solution3(&s1, &s2);
    println!("s1 and s2 is anagram: {result}");
}
