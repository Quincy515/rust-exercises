/// UTF-8 & 索引
/// 由于 String 都是 UTF-8 编码的，这会带来几个影响:

/// - 如果你需要的是非 UTF-8 字符串，可以考虑 OsString
/// - 无法通过索引的方式访问一个 String

/// 具体请看字符串索引。

/// 4. 🌟🌟🌟 我们无法通过索引的方式访问字符串中的某个字符，
/// 但是可以通过切片的方式来获取字符串的某一部分 &s1[start..end]


// 填空并修复错误
fn main() {
    let s = String::from("hello, 世界");
    let slice1 = &s[..1]; //提示: `h` 在 UTF-8 编码中只占用 1 个字节
    assert_eq!(slice1, "h");

    let slice2 = &s[7..10];// 提示: `中` 在 UTF-8 编码中占用 3 个字节
    assert_eq!(slice2, "世");
    
    // 迭代 s 中的所有字符
    for (i, c) in s.chars().enumerate() {
        if i == 7 {
            assert_eq!(c, '世')
        }
    }

    println!("Success!")
}

