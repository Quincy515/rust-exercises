// use mod_05::mod_d::{animal::*, dog::*};
use mod_05::mod_d::*;
use mod_05::{mod_a::*, mod_b::*, mod_c::*};
use mod_05::animal;

macro_rules! map {
    ($($key:expr => $value:expr), *) => {
        {
            let mut hash_map = std::collections::HashMap::new();
            $(hash_map.insert($key, $value);)*
            hash_map
        }
    }
}

fn main() {
    a_say();
    b_say();
    c_say();
    let animal = Animal::new("animalA", 8);
    let dog = Dog::new("dogA", 9);
    animal.say();
    dog.say();

    test_animals();
    let animal_d = animal!("frog", 11);
    animal_d.say();

    // vec!["a", "b"]
    // let id_name_map = {let hash_map...}
    let id_name_map = map!(
        1 => "cat",
        2 => "dog",
        3 => "lion"
    );
    println!("id_name: {:?}", id_name_map);
}

