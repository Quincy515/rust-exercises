use std::rc::Rc; // 将 Rc<T> 引入作用域

fn main() {
    let x = Rc::new(5); // 创建一个 Rc<i32> 类型的值，并与变量 x 绑定
    println!(
        "{:p}, count after constructing x: {}",
        x,
        Rc::strong_count(&x) // 获取当前的引用计数
    );

    let y = x.clone(); // 以 clone 方法克隆了 x，并与变量 y 绑定
    println!(
        "{:p}, count after constructing y: {}",
        y,
        Rc::strong_count(&x)
    );

    {
        // 一般使用 Rc::clone 函数，表示对共享所有权进行计数，并非对数据进行深复制
        let z = Rc::clone(&x); // 以 Rc::clone 函数克隆了 x，并与变量 z 绑定
        println!(
            "{:p}, count after constructing z: {}",
            z,
            Rc::strong_count(&x)
        );
    }

    println!("count after destructing z: {}", Rc::strong_count(&x));
}
