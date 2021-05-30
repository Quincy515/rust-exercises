use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;

// 同一作用域只允许有多个 Ref<T> 或一个 RefMut<T>
// RefCell<Vec<i32>> 通过 borrow_mut 方法创建了两个 RefMut<Vec<i32>>
// 程序运行会抛出 already borrowed：BorrowMutError 的错误提示
fn main() {
    let v: RefCell<Vec<i32>> = RefCell::new(vec![1, 2, 4]);

    let mut v_borrow_mut_1: RefMut<Vec<i32>> = v.borrow_mut();
    v_borrow_mut_1.push(5);
    println!("{:?}", v_borrow_mut_1); // [1, 2, 4]

    let mut v_borrow_mut_2: RefMut<Vec<i32>> = v.borrow_mut();
    v_borrow_mut_2.push(6);
    println!("{:?}", v_borrow_mut_2);
}
