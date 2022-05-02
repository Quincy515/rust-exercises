// Trait Bounds
// Just like generic types can be bounded, lifetimes can also be bounded as below:

// T: 'a，all references in T must outlive the lifetime 'a
// T: Trait + 'a: T must implement trait Trait and all references in T must outlive 'a
// Example
use std::fmt::Debug; // Trait to bound with.

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
// `Ref` contains a reference to a generic type `T` that has
// an unknown lifetime `'a`. `T` is bounded such that any
// *references* in `T` must outlive `'a`. Additionally, the lifetime
// of `Ref` may not exceed `'a`.

// A generic function which prints using the `Debug` trait.
fn print<T>(t: T) where
    T: Debug {
    println!("`print`: t is {:?}", t);
}

// Here a reference to `T` is taken where `T` implements
// `Debug` and all *references* in `T` outlive `'a`. In
// addition, `'a` must outlive the function.
fn print_ref<'a, T>(t: &'a T) where
    T: Debug + 'a {
    println!("`print_ref`: t is {:?}", t);
}

fn demo() {
    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);
}

// 1、🌟

/* Annotate struct with lifetime:
1. `r` and `s` must has different lifetimes
2. lifetime of `s` is bigger than that of 'r'
*/
struct DoubleRef<'a, 'b: 'a, T> {
    r: &'a T,
    s: &'b T
}
fn main() {
    demo();
    println!("Success!")
}
