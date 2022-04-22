
fn main() {
    let x = Box::new(5);
    
    let mut y = Box::new(3);      // 完成该行代码，不要修改其它行！
    
    *y = 4;
    
    assert_eq!(*x, 5);
}

