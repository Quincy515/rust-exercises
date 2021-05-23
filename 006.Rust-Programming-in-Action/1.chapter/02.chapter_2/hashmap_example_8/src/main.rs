use std::collections::HashMap;

fn main() {
    // 哈希表（HashMap）是基于哈希表来存储键-值对的集合，
    // 其中所有的键必须是同一类型，所有的值也必须是同一类型，不允许有重复的键
    // 但允许不同的键有相同的值
    // 1. HashMap 的创建
    // 使用 HashMap::new 函数创建空的 HashMap
    let mut map: HashMap<&str, i32> = HashMap::new();
    // 使用 HashMap::with_capacity 函数创建指定容量的 HashMap
    let mut map: HashMap<&str, i32> = HashMap::with_capacity(10);

    // 2. HashMap 的修改
    // 使用 insert 方法在 HashMap 中插入或更新键值对。
    // 如果键不存在，执行插入操作并返回 None。
    // 如果键已存在，执行更新操作，将对应键的值更新并返回旧值。
    let mut map: HashMap<&str, i32> = HashMap::new();
    let zhangsan1 = map.insert("zhangsan", 97);
    map.insert("lisi", 86);

    println!("{:?}", zhangsan1); // None
    println!("{:?}", map); // {"lisi": 86, "zhangsan": 97}

    let zhangsan2 = map.insert("zhangsan", 79);
    println!("{:?}", zhangsan2); // Some(97)
    println!("{:?}", map); // {"lisi": 86, "zhangsan": 79}

    // 使用 entry 和 or_insert 方法检查键是否有对应值，
    // 没有对应值就插入键-值对，已有对应值则不执行任何操作。
    // entry 方法以键为参数，返回值是一个枚举类型 Entry。
    // Entry 类型的 or_insert 方法以值为参数，在键有对应值时不执行任何操作。
    // 在键没有对应值时，将键与值组成键-值对插入 HashMap。
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.entry("zhangsan").or_insert(97);
    map.entry("lisi").or_insert(86);
    println!("{:?}", map); // {"lisi": 86, "zhangsan": 97}

    map.entry("zhangsan").or_insert(79);
    println!("{:?}", map); // {"lisi": 86, "zhangsan": 97}

    // 以新旧两值的计算结果来更新键值对
    // iter_mut 方法返回有 HashMap 中所有键-值对的可变引用组成的迭代器
    // 通过 for 循环遍历迭代器，通过解引用操作符"*"对val进行赋值操作
    let mut map: HashMap<&str, i32> = HashMap::new();

    map.insert("zhangsan", 97);
    map.insert("lisi", 86);
    map.insert("wangwu", 55);
    println!("{:?}", map); // {"zhangsan": 97, "lisi": 86, "wangwu": 55}

    for (_, val) in map.iter_mut() {
        *val += 2;
    }
    println!("{:?}", map); // {"zhangsan": 99, "lisi": 88, "wangwu": 57}

    // 使用 remove 方法删除键-值对
    let mut map: HashMap<&str, i32> = HashMap::new();

    map.insert("zhangsan", 97);
    map.insert("lisi", 86);
    map.insert("wangwu", 55);
    println!("{:?}", map); // {"zhangsan": 97, "lisi": 86, "wangwu": 55}

    let result = map.remove("wangwu");
    println!("{:?}", map); // {"zhangsan": 97, "lisi": 86}
    println!("{:?}", result); // Some(55)

    // 3. HashMap 的访问
    // 使用"实例名[键]"语法访问指定的键-值对。
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("zhangsan", 97);

    println!("zhangsan: {}", map["zhangsan"]); // zhangsan: 97

    // println!("wangwu: {}", map["wangwu"]);

    // 使用 get 方法以键为参数访问指定的键-值对
    println!("zhangsan: {:?}", map.get("zhangsan")); // zhangsan: Some(97)
    println!("wangwu: {:?}", map.get("wangwu")); // wangwu: None
}
