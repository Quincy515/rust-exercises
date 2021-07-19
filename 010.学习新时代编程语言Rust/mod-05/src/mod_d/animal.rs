#[derive(Debug)]
pub struct Animal {
  name: &'static str,
  age: u8,
}

impl Animal {
  pub fn new(name: &'static str, age: u8) -> Self {
      Self { name, age }
  }
  pub fn say(&self) {
      println!("aniaml {} say age {}", self.name, self.age);
  }
}