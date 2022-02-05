/// ASCII 哈希函数
/// 对每个字符的 ascii 值求和，再对插槽求余
fn hash1(astr: &str, size: usize) -> usize {
    let mut sum = 0;
    for c in astr.chars() {
        sum += c as usize;
    }

    sum % size
}

/// 上面的hash函数冲突比较严重，可以使用不同字符的位置为权重
/// 将其 ascii 值乘以其位置权重
fn hash2(astr: &str, size: usize) -> usize {
    let mut sum = 0;
    for (i, c) in astr.chars().enumerate() {
        sum += (i + 1) * (c as usize);
    }
    sum % size
}

fn main() {
    let size = 11;
    let s1 = "rust";
    let s2 = "Rust";
    let p1 = hash1(s1, size);
    let p2 = hash1(s2, size);
    println!("{s1} in slot {p1}, {s2} in slot {p2}");
}
