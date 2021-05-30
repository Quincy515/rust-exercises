use std::cell::Ref;
use std::cell::RefCell;

// 同一作用域只允许有多个 Ref<T> 或一个 RefMut<T>
// RefCell<Vec<i32>> 通过 borrow 方法创建了两个 Ref<Vec<i32>>
fn main() {
    let v: RefCell<Vec<i32>> = RefCell::new(vec![1, 2, 4]);

    let v_borrow_1: Ref<Vec<i32>> = v.borrow();
    println!("{:?}", v_borrow_1); // [1, 2, 4]

    let v_borrow_2: Ref<Vec<i32>> = v.borrow();
    println!("{:?}", v_borrow_2); // [1, 2, 4]
}
