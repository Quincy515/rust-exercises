/// utf8_slice
/// 我们可以使用 utf8_slice 来按照字符的自然索引方式对 UTF-8 字符串进行切片访问，
/// 与之前的切片方式相比，它索引的是字符，而之前的方式索引的是字节.

/// 示例

/*
use utf8_slice;
fn main() {
   let s = "The 🚀 goes to the 🌑!";

   let rocket = utf8_slice::slice(s, 4, 5);
   // Will equal "🚀"
}
*/

/// 5. 🌟🌟🌟
/// 提示: 也许你需要使用 from_utf8 方法



// 填空
fn main() {
    let mut s = String::new();
    s.push_str("hello");

    let v = vec![104, 101, 108, 108, 111];

    // 将字节数组转换成 String
    let s1 = std::str::from_utf8(&v).unwrap();
    
    
    assert_eq!(s, s1);

    println!("Success!")
}

