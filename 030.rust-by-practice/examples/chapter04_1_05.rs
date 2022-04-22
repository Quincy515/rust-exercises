///
///// 解决代码中的错误和 `panic`
///// 解决代码中的错误和 `panic`
///fn main() {
///   let v1 = 251_u8 + 8;
///   let v2 = i8::checked_add(251, 8).unwrap();
///   println!("{},{}",v1,v2);
///}


// 解决代码中的错误和 `panic`
fn main() {
   let v1 = 247_u8 + 8;
   let v2 = i8::checked_add(119, 8).unwrap();
   println!("{},{}",v1,v2);
}

