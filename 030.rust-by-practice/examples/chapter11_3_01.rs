/// HashMap
/// HashMap 默认使用 SipHash 1-3 哈希算法，该算法对于抵抗 HashDos 攻击非常有效。
/// 在性能方面，如果你的 key 是中型大小的，那该算法非常不错，
/// 但是如果是小型的 key( 例如整数 )亦或是大型的 key ( 例如字符串 )，
/// 那你需要采用社区提供的其它算法来提高性能。

/// 哈希表的算法是基于 Google 的 SwissTable，
/// 你可以在这里找到 C++ 的实现，
/// 同时在 CppCon talk 上也有关于算法如何工作的演讲。

/// 基本操作
/// 1. 🌟🌟


// 填空并修复错误
use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert("Sunface", 98);
    scores.insert("Daniel", 95);
    scores.insert("Ashley", 69);
    scores.insert("Katie", 58);

    // get 返回一个 Option<&V> 枚举值
    let score = scores.get("Sunface");
    assert_eq!(score, Some(&98));

    if scores.contains_key("Daniel") {
        // 索引返回一个值 V
        let score = scores["Daniel"];
        assert_eq!(score, 95);
        scores.remove("Daniel");
    }

    assert_eq!(scores.len(), 3);

    for (name, score) in scores {
        println!("The score of {} is {}", name, score)
    }
}

