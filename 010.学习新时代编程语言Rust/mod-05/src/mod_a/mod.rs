pub fn a_say() {
  println!("a_say");
  mod_a1::a1_say(); // 父模块不能访问子模块的私有成员
}
fn a_private_say() {
  println!("a_private_say");
}
mod mod_a1 {
  pub fn a1_say() {
    println!("a1_say");
    crate::mod_a::a_private_say(); // 绝对路径
    super::a_private_say(); // 子模块可以访问父模块的私有成员
  }
}