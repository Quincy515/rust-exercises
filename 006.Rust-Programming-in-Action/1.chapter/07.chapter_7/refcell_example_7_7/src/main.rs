use std::cell::RefCell;

fn main() {
    let v: RefCell<Vec<i32>> = RefCell::new(vec![1, 2, 4]);
    println!("{:?}", v.borrow()); // RefCell 提供了 borrow 方法返回 Ref 类型的智能指针

    v.borrow_mut().push(5); // borrow_mut 方法返回 RefMut 类型的智能指针
    println!("{:?}", v.borrow());
}
