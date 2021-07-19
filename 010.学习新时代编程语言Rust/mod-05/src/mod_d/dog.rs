pub struct Dog {
  name: &'static str,
  age: u8,
}

impl Dog {
  pub fn new(name: &'static str, age: u8) -> Self {
      Self { name, age }
  }
  pub fn say(&self) {
      println!("dog {} say age {}", self.name, self.age);
  }
}
