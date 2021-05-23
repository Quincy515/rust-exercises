程序中常用的三大数据结构包括：

- 动态数组
- 映射
- 字符串

Rust 标准库 std::collections 提供了 4 种通用的容器类型，其中包括 8 种数据结构。

- Sequences: [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html), [`VecDeque`](https://doc.rust-lang.org/std/collections/struct.VecDeque.html), [`LinkedList`](https://doc.rust-lang.org/std/collections/struct.LinkedList.html)
- Maps: [`HashMap`](https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html), [`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html)
- Sets: [`HashSet`](https://doc.rust-lang.org/std/collections/hash_set/struct.HashSet.html), [`BTreeSet`](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html)
- Misc: [`BinaryHeap`](https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html)

变量和可变性

Rust 的变量本质上是一种绑定语义，即将一个变量名与一个值绑定在一起。变量名和值建立关联关系。

基本数据类型：

- 整数类型
- 浮点数类型
- 布尔类型
- 字符类型
- 范围类型

复合数据类型：

- 元组: 一个或多个类型的元素组合，使用小括号把所有元素放在一起。元素之间使用逗号分隔。
- 数组: 相同类型的元素组合，使用 [T;n] 表示，T 代表元素类型，n 代表长度即元素个数。
- 结构体: 一个自定义数据类型，通过 struct 关键字加自定义命名，可以把多个类型组合在一起成为新的类型。
  - 元组结构体是指字段只有类型，没有名称。 struct Color(i32, i32, i32); let black = Color(0, 0, 0);
  - 单元结构体是指没有任何字段的结构体。struct Solution;
- 枚举: 自定义数据类型，通过 enum 关键字加自定义命名来定义。
  - 无参数枚举类型、带参数枚举类型

容器类型：

- 线性序列: 
  - [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html), 连续存储的可变长数组（简称动态数组）
    - let mut v: Vec<i32> = Vec::new();
    - let mut v: Vec<i32> = Vec::with_capacity(10);
    - let mut v = vec![0;10]; vec![1,2,3]; vec![];
  -  [`VecDeque`](https://doc.rust-lang.org/std/collections/struct.VecDeque.html), 连续存储的可变长双端队列
  -  [`LinkedList `](https://doc.rust-lang.org/std/collections/struct.LinkedList.html) 非连续存储的双向链表
- 键-值对: 
  - [`HashMap`](https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html),  基于哈希表的无序键-值对
  - [`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html)基于 B 树的有序键-值对，按 Key 排序
- 集合Sets:
  -  [`HashSet`](https://doc.rust-lang.org/std/collections/hash_set/struct.HashSet.html), 基于哈希表的无序集合
  - [`BTreeSet`](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html) 基于 B 树的有序集合
- 优先队列Misc: [`BinaryHeap`](https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html) 基于二叉堆的优先队列

字符串：本质是一种特殊的容器类型，是由零个或多个字符组成的有限序列。

- 固定长度的字符串字面量 str
- 可变程度的字符串对象 String