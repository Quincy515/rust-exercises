// NLL (Non-Lexical Lifetime)
// Before explaining NLL, let's see some code first:
#![allow(unused)]
fn demo1() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);
}

// Based on our current knowledge,
// this code will cause en error due to violating the borrowing rules in Rust.

// But if you cargo run it, then everything will be ok, so what's going on here?

// The ability of the compiler to tell
// that a reference is no longer used at a point
// before the end of the scope, is called Non-Lexical Lifetimes (NLL for short).

// With this ability the compiler knows
// when is the last time that a reference is used
// and optimizing the borrowing rules based on this knowledge.

/**
let mut u = 0i32;
let mut v = 1i32;
let mut w = 2i32;

// lifetime of `a` = α ∪ β ∪ γ
let mut a = &mut u;     // --+ α. lifetime of `&mut u`  --+ lexical "lifetime" of `&mut u`,`&mut u`, `&mut w` and `a`
use(a);                 //   |                            |
*a = 3; // <-----------------+                            |
...                     //                                |
a = &mut v;             // --+ β. lifetime of `&mut v`    |
use(a);                 //   |                            |
*a = 4; // <-----------------+                            |
...                     //                                |
a = &mut w;             // --+ γ. lifetime of `&mut w`    |
use(a);                 //   |                            |
*a = 5; // <-----------------+ <--------------------------+
 */

// Reborrow
// After learning NLL, we can easily understand reborrow now.

// Example

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

fn demo2() {
    let mut p = Point { x: 0, y: 0 };
    let r = &mut p;
    // Here comes the reborrow
    let rr: &Point = &*r;

    println!("{:?}", rr); // Reborrow ends here, NLL introduced

    // Reborrow is over, we can continue using `r` now
    r.move_to(10, 10);
    println!("{:?}", r);
}

// 5、🌟🌟
/* Make it work by reordering some code */
fn main() {
    let mut data = 10;
    let ref1 = &mut data;
    let ref2 = &mut *ref1;

    *ref2 += 2;
    *ref1 += 1;

    println!("{}", data);
}
