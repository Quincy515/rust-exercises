/*
pub mod animal;
pub mod dog;
pub use animal::Animal;
pub use dog::Dog;
*/
include!("./animal.rs");
include!("./dog.rs");

/// 定义宏
#[macro_export] // 相当于 pub fn 公开
macro_rules! animal {
  ($name: expr) => {
    Animal::new($name, 0)
  };
  ($name:expr, $age:expr) => {
    Animal::new($name, $age)
  }
}

pub fn test_animals() {
  let animal_a = animal!("cat");
  let animal_b = animal!("fox", 8);
  animal_a.say();
  animal_b.say();

}