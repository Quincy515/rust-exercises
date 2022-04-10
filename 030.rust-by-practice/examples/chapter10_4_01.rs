/// 特征对象
/// 在特征练习中 我们已经知道当函数返回多个类型时，impl Trait 是无法使用的。

/// 对于数组而言，其中一个限制就是无法存储不同类型的元素，
/// 但是通过之前的学习，大家应该知道枚举可以在部分场景解决这种问题，
/// 但是这种方法局限性较大。此时就需要我们的主角登场了。

/// 使用 dyn 返回特征
/// Rust 编译器需要知道一个函数的返回类型占用多少内存空间。
/// 由于特征的不同实现类型可能会占用不同的内存，
/// 因此通过 impl Trait 返回多个类型是不被允许的，
/// 但是我们可以返回一个 dyn 特征对象来解决问题。

/// 1. 🌟🌟🌟


trait Bird {
    fn quack(&self) -> String;
}

struct Duck;
impl Duck {
    fn swim(&self) {
        println!("Look, the duck is swimming")
    }
}
struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) -> String{
        "duck duck".to_string()
    }
}

impl Bird for Swan {
    fn quack(&self) -> String{
        "swan swan".to_string()
    }
}

fn main() {
    // 填空
    let duck = Duck{};
    duck.swim();

    let bird = hatch_a_bird(2);
    // 变成鸟儿后，它忘记了如何游，因此以下代码会报错
    // bird.swim();
    // 但它依然可以叫唤
    assert_eq!(bird.quack(), "duck duck");

    let bird = hatch_a_bird(1);
    // 这只鸟儿忘了如何飞翔，因此以下代码会报错
    // bird.fly();
    // 但它也可以叫唤
    assert_eq!(bird.quack(), "swan swan");

    println!("Success!")
}   

// 实现以下函数
fn hatch_a_bird(n: i32) -> Box<dyn Bird> {
    if n == 1 {
        return Box::new(Swan{});
    }
    return Box::new(Duck{});
}


