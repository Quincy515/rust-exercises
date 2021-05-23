fn main() {
    let mut v: Vec<i32> = Vec::new();
    // 添加元素，使用 push 方法将值添加到动态数组的尾部
    v.push(1);
    v.push(2);
    v.push(3);

    println!("v: {:?}", v); // [1, 2, 3]

    // 使用“实例名[索引]”语法为指定索引的元素重新赋值
    v[1] = 5;
    println!("v: {:?}", v); // [1, 5, 3]

    // 使用 pop 方法删除并返回动态数组的最后一个元素，如果数组为空则返回 None
    let mut v: Vec<i32> = Vec::with_capacity(10);
    v.push(1);
    v.push(2);
    v.push(3);

    println!("e: {:?}", v.pop()); // e: Some(3)
    println!("v: {:?}", v); // v: [1, 2]

    // 使用 remove 方法删除并返回动态数组指定索引的元素，同时将其后面的所有元素向左移动一位
    let mut v = vec![1, 2, 3];
    println!("e: {}", v.remove(1)); // e: 2
    println!("v: {:?}", v); // v: [1, 3]

    // 使用"实例名[索引]"语法访问指定索引的元素
    let v = vec![1, 2, 3];
    println!("e: {}", v[2]); // e: 3

    // println!("e: {}", v[10]);

    // 使用 get 方法以索引作为参数访问元素
    let v = vec![1, 2, 3];
    println!("e:{:?}", v.get(2)); // e: Some(3) 返回值类型是 Option
    println!("e:{:?}", v.get(10)); // e: None 由于索引已越界，将返回 None

    // 使用 for 循环遍历访问动态数组的所有元素
    let v = vec![10, 20, 30];
    for i in v {
        print!("{} ", i);
    }

    // 使用 for 循环遍历动态数组中每一个元素的可变引用，并使用解引用操作符"*"来追踪和修改元素值。
    let mut v = vec![10, 20, 30];
    for i in &mut v {
        *i += 50;
        print!("{} ", i);
    }
}
