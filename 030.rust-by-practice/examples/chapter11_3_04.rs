/// HashMap key 的限制
/// 任何实现了 Eq 和 Hash 特征的类型都可以用于 HashMap 的 key，包括:

/// bool (虽然很少用到，因为它只能表达两种 key)
/// int, uint 以及它们的变体，例如 u8、i32 等
/// String 和 &str (提示: HashMap 的 key 是 String 类型时，你其实可以使用 &str 配合 get 方法进行查询
/// 需要注意的是，f32 和 f64 并没有实现 Hash，原因是 浮点数精度 的问题会导致它们无法进行相等比较。

/// 如果一个集合类型的所有字段都实现了 Eq 和 Hash,那该集合类型会自动实现 Eq 和 Hash。
/// 例如 Vect<T> 要实现 Hash，那么首先需要 T 实现 Hash。

/// 🌟🌟


// 修复错误
// 提示: `derive` 是实现一些常用特征的好办法
use std::collections::HashMap;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Viking {
    name: String,
    country: String,
}

impl Viking {
    fn new(name: &str, country: &str) -> Viking {
        Viking {
            name: name.to_string(),
            country: country.to_string(),
        }
    }
}

fn main() {
    // 使用 HashMap 来存储 viking 的生命值
    let vikings = HashMap::from([
        (Viking::new("Einar", "Norway"), 25),
        (Viking::new("Olaf", "Denmark"), 24),
        (Viking::new("Harald", "Iceland"), 12),
    ]);

    // 使用 derive 的方式来打印 vikong 的当前状态
    for (viking, health) in &vikings {
        println!("{:?} has {} hp", viking, health);
    }
}

