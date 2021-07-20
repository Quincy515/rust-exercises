use procedure_derive::Hello;
use procedure_derive::say_hello;
use procedure_derive::hello_custer;

hello_custer!();

trait SayHello {
    fn say_hello(&self);
}

#[say_hello(WangWang)]
pub struct Dog {}

// impl SayHello for Dog {
//     fn say_hello(&self) {
//         println!("{} say Hello", "Dog");
//     }
// }

#[derive(Hello)]
pub struct Cat {}

#[say_hello(Gege)]
pub struct Chicken {}

fn main() {
    let dog = Dog {};
    dog.say_hello();
    let cat = Cat {};
    cat.say_hello();
    let chicken = Chicken {};
    chicken.say_hello();

    hello_custer();
}
