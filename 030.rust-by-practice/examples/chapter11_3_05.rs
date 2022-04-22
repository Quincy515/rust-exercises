/// 容量
/// 关于容量，我们在之前的 Vector 中有详细的介绍，
/// 而 HashMap 也可以调整容量: 
/// 你可以通过 HashMap::with_capacity(uint) 使用指定的容量来初始化，
/// 或者使用 HashMap::new() ，后者会提供一个默认的初始化容量。

/// 示例

// use std::collections::HashMap;
fn main_demo() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);
    // 事实上，虽然我们使用了 100 容量来初始化，但是 map 的容量很可能会比 100 更多
    assert!(map.capacity() >= 100);

    // 对容量进行收缩，你提供的值仅仅是一个允许的最小值，实际上，Rust 会根据当前存储的数据量进行自动设置，当然，这个值会尽量靠近你提供的值，同时还可能会预留一些调整空间

    map.shrink_to(50);
    assert!(map.capacity() >= 50);

    // 让 Rust  自行调整到一个合适的值，剩余策略同上
    map.shrink_to_fit();
    assert!(map.capacity() >= 2);
    println!("Success!")
}

/// 所有权
/// 对于实现了 Copy 特征的类型，例如 i32，那类型的值会被拷贝到 HashMap 中。
/// 而对于有所有权的类型，例如 String，它们的值的所有权将被转移到 HashMap 中。

/// 5. 🌟🌟


// 修复错误，尽可能少的去修改代码
// 不要移除任何代码行！
use std::collections::HashMap;
fn main() {
    main_demo();

  let v1 = 10;
  let mut m1 = HashMap::new();
  m1.insert(v1, v1);
  println!("v1 is still usable after inserting to hashmap : {}", v1);

  let v2 = "hello".to_string();
  let mut m2 = HashMap::new();
  // 所有权在这里发生了转移
  m2.insert(v2.clone(), v1);

  assert_eq!(v2, "hello");

   println!("Success!")
}

/*
/// 三方库 Hash 库
/// 在开头，我们提到过如果现有的 SipHash 1-3 的性能无法满足你的需求，那么可以使用社区提供的替代算法。

/// 例如其中一个社区库的使用方式如下：

#![allow(unused)]
fn main_hash() {
use std::hash::BuildHasherDefault;
use std::collections::HashMap;
// 引入第三方的哈希函数
use twox_hash::XxHash64;


let mut hash: HashMap<_, _, BuildHasherDefault<XxHash64>> = Default::default();
hash.insert(42, "the answer");
assert_eq!(hash.get(&42), Some(&"the answer"));
}

*/
