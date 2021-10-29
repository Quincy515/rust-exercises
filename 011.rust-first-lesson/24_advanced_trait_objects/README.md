## trait object 

### 好处

当在某个上下文中需要满足某个 trait 的类型，且这样的类型可能有很多，当前上下文无法确定会得到哪一个类型时，我们可以用 trait object 来统一处理行为

和泛型参数一样，trait object 也是一种延迟绑定，它让决策可以延迟到运行时，从而得到最大的灵活性。

详情阅读 Rust book [Using Trait Objects that allow for values of different types](https://doc.rust-lang.org/book/ch17-02-trait-objects.html)

## 在函数参数中使用 trait object

阅读 `trait_object_in_fn.rs` 代码

## 在函数返回值中使用 trait object

这是 trait object 使用频率比较高的场景

阅读 `lib.rs` 代码

## 在数据结构中使用 trait object

大部分时候处理这样的数据结构的选择。

但是，过多的泛型参数会带来两个问题：

- 首先，代码实现过程中，所有涉及的接口都变得非常臃肿
- 其次，这些参数所有被使用到的情况，组合起来，会生成大量的代码。

而使用 trait object，我们在牺牲一点性能的前提下，消除了这些泛型参数，实现的代码更干净清爽，且代码只会有一份实现。

在数据结构中使用 trait object 还有一种很典型的场景是，闭包。

阅读 `lib.rs` 代码

### 小结

作为 Rust 开发者，我们不介意泛型参数带来的稍微复杂的代码结构，
愿意用开发时的额外付出，换取运行时的高效；

但有时候，当泛型参数过多，导致代码出现了可读性问题，
或者运行效率并不是主要矛盾的时候，
我们可以通过使用 trait object 做动态分发，来降低代码的复杂度。