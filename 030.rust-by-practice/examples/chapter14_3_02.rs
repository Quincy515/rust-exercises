/// 2. 🌟🌟 如果我们在使用来自同一个包或模块中的多个不同项，
/// 那么可以通过简单的方式将它们一次性引入进来




// 使用两种方式填空
// 不要添加新的代码行
use std::collections::{HashMap,HashSet, BTreeMap};

fn main() {
    let _c1:HashMap<&str, i32> = HashMap::new();
    let mut c2 = BTreeMap::new();
    c2.insert(1, "a");
    let _c3: HashSet<i32> = HashSet::new();
}

