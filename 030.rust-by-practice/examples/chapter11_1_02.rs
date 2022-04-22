/// String and &str
/// 虽然 String 的底层是 Vec<u8> 也就是字节数组的形式存储的，
/// 但是它是基于 UTF-8 编码的字符序列。
/// String 分配在堆上、可增长且不是以 null 结尾。

/// 而 &str 是切片引用类型( &[u8] )，指向一个合法的 UTF-8 字符序列，
/// 总之，&str 和 String 的关系类似于 &[T] 和 Vec<T> 。

/// 如果大家想了解更多，可以看看易混淆概念解析 - &str 和 String。

/// 2. 🌟🌟

// 填空
fn main() {  
   let mut s = String::from("hello, world");

   // let slice1: &str = &s; // 使用两种方法
   let slice1: &str = s.as_str(); // 使用两种方法
   assert_eq!(slice1, "hello, world");

   let slice2 = &s[..5];
   assert_eq!(slice2, "hello");

   let slice3: &mut String = &mut s; 
   slice3.push('!');
   assert_eq!(slice3, "hello, world!");

   println!("Success!")
}

