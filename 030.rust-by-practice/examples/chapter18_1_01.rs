// Closure
// Closures can capture the enclosing evironments.
// For example we can capture the x variable :

fn demo1() {
    let x = 1;
    let closure = |val| val + x;
    assert_eq!(closure(2), 3);
}

// From the syntax,
// we can see that closures are very convenient for on the fly usage.
// Unlike functions,
// both the input and return types of a closure can be inferred by the compiler.

fn demo2() {
    // Increment via closures and functions.
    fn function(i: i32) -> i32 {
        i + 1
    }

    // Closures are anonymous, here we are binding them to references
    //
    // These nameless functions are assigned to appropriately named variables.
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;

    let i = 1;
    // Call the function and closures.
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    let one = || 1;
    println!("closure returning one: {}", one());
}

// Capturing
// Closures can capture variables by borrowing or moving.
// But they prefer to capture by borrowing and only go lower when required:

// - by reference: &T
// - by mutable reference: &mut T
// - by value: T

// 1、🌟
/* Make it work with least changing */
fn main() {
    let color = String::from("green");

    let print =   || println!("`color`: {}", color);

    print();
    print();

    // `color` can be borrowed immutably again, because the closure only holds
    // an immutable reference to `color`.
    let _reborrow = &color;

    println!("{}", color);
    demo1();
    demo2();
}
