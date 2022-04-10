use my_serde::{serde_derive, serde_std};

fn main() {
    serde_derive::serde_derive();
    serde_std::serde_std();
    println!("Success");
}
