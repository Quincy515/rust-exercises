use std::collections::VecDeque;

fn main() {
    // 1. VecDeque 的创建
    // 使用 VecDeque::new 函数创建空的 VecDeque
    let mut v: VecDeque<u32> = VecDeque::new();
    // 使用 VecDeque::with_capacity 函数创建指定容量的 VecDeque
    let mut v: VecDeque<u32> = VecDeque::with_capacity(10);

    // 2. 添加元素
    // 使用 push_front 方法在队列的头部添加新元素
    // 使用 push_back 方法在队列的尾部添加新元素
    let mut v: VecDeque<u32> = VecDeque::new();
    v.push_back(1);
    v.push_back(2);
    v.push_back(3);
    v.push_front(1);
    v.push_front(2);

    println!("V: {:?}", v); // v: [2, 1, 1, 2, 3]

    // 3. 修改指定索引的元素值
    // 使用"实例名[索引]语法为指定索引的元素重新赋值"
    let mut v: VecDeque<u32> = VecDeque::new();
    v.push_back(1);
    v.push_back(2);
    v.push_back(3);

    v[1] = 5;
    println!("v: {:?}", v); // v: [1, 5, 3]

    // 4. 删除 VecDeque 的元素
    // 使用 pop_front 方法删除并返回队列的头部元素，使用 pop_back 方法删除并返回队列的尾部元素
    let mut v: VecDeque<u32> = VecDeque::new();
    v.push_back(1);
    v.push_back(2);
    v.push_back(3);
    v.push_front(1);
    v.push_front(2);

    println!("e: {:?}", v.pop_back()); // e: Some(3)
    println!("e: {:?}", v.pop_front()); // e: Some(2)
    println!("v: {:?}", v); // v: [1, 1, 2]

    // 使用 remove 方法删除并返回队列指定索引的元素，同时将其后面的所有元素向左移动一位。
    let mut v: VecDeque<u32> = VecDeque::with_capacity(10);
    v.push_back(1);
    v.push_back(2);
    v.push_back(3);

    println!("e: {:?}", v.remove(1)); // e: Some(2)
    println!("e: {:?}", v.remove(5)); // e: None
    println!("v: {:?}", v); // v: [1, 3]

    // 5. 使用索引语法访问元素
    // 使用"实例名[索引]"语法访问指定索引的元素
    let mut v: VecDeque<u32> = VecDeque::new();
    v.push_back(1);
    v.push_back(2);
    v.push_back(3);

    println!("e: {}", v[0]); // e: 1
                             // println!("e: {}", v[10]);

    // 使用 get 方法以索引作为参数访问元素
    let mut v: VecDeque<u32> = VecDeque::new();
    v.push_back(1);
    v.push_back(2);
    v.push_back(3);

    println!("e: {:?}", v.get(0)); // e: Some(1)
    println!("e: {:?}", v.get(10)); // e: None
}
