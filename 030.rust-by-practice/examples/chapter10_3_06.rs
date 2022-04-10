/// 使用特征作为函数返回值
/// 我们还可以在函数的返回值中使用 impl Trait 语法。
/// 然后只有在返回值是同一个类型时，才能这么使用，如果返回值是不同的类型，你可能更需要特征对象。

/// 6. 🌟🌟



struct Sheep {}
struct Cow {}

trait Animal {
    fn noise(&self) -> String;
}

impl Animal for Sheep {
    fn noise(&self) -> String {
        "baaaaah!".to_string()
    }
}

impl Animal for Cow {
    fn noise(&self) -> String {
        "moooooo!".to_string()
    }
}

// 返回一个类型，该类型实现了 Animal 特征，但是我们并不能在编译期获知具体返回了哪个类型
// 修复这里的错误，你可以使用虚假的随机，也可以使用特征对象
fn random_animal_impl(random_number: f64) -> impl Animal {
    if random_number < 0.5 {
        Sheep {}
    } else {
        // Cow {}
        Sheep {}
    }
}


fn random_animal_box(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let random_number = 0.234;
    let animal = random_animal_impl(random_number);
    let animal_box = random_animal_box(random_number + 1.0);
    println!("You've randomly chosen an animal, and it says {}, {}", animal.noise(), animal_box.noise());
}

