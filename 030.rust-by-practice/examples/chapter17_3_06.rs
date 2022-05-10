// Unbound lifetime
// See more info in [Nomicon - Unbounded Lifetimes.](https://doc.rust-lang.org/nomicon/unbounded-lifetimes.html)

// More elision rules
/**
impl<'a> Reader for BufReader<'a> {
    // 'a is not used in the following methods
}

// can be writting as :
impl Reader for BufReader<'_> {

}
 */

/**
// Rust 2015
struct Ref<'a, T: 'a> {
    field: &'a T
}

// Rust 2018
struct Ref<'a, T> {
    field: &'a T
}
  */

// A difficult exercise
// 6、🌟🌟🌟🌟

/* Make it work */
struct Interface<'a> {
    manager: &'a mut Manager<'a>,
}

impl<'a> Interface<'a> {
    pub fn noop(self) {
        println!("interface consumed");
    }
}

struct Manager<'a> {
    text: &'a str,
}

struct List<'a> {
    manager: Manager<'a>,
}

impl<'a> List<'a> {
    pub fn get_interface(&'a mut self) -> Interface {
        Interface {
            manager: &mut self.manager,
        }
    }
}

fn main() {
    let mut list = List {
        manager: Manager { text: "hello" },
    };

    use_list(&list);

    list.get_interface().noop();

    println!("Interface should be dropped here and the borrow released");
}

fn use_list(list: &List) {
    println!("{}", list.manager.text);
}
