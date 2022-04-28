enum List {
    Cons(i32, Box<List>),
    Nil,
}

// Box::leak



fn main() {
    let a = Box::new(3);
    let b = &3;
    let x = &*a;
    println!("a = {b}");
}
