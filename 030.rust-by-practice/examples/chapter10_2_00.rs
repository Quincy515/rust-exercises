/// Const 泛型
/// 在之前的泛型中，
/// 可以抽象为一句话：针对类型实现的泛型，所有的泛型都是为了抽象不同的类型，
/// 那有没有针对值的泛型？答案就是 Const 泛型。

/// 示例
/// 1. 下面的例子同时使用泛型和 const 泛型来实现一个结构体，该结构体的字段中的数组长度是可变的

use std::fmt::Debug;

struct ArrayPair<T, const N: usize> {
    left: [T; N],
    right: [T; N],
}

impl<T: Debug, const N: usize> Debug for ArrayPair<T, N> {
    // ...
}

/// 2. 目前，const 泛型参数只能使用以下形式的实参:
/// 一个单独的 const 泛型参数
/// 一个字面量 (i.e. 整数, 布尔值或字符).
/// 一个具体的 const 表达式( 表达式中不能包含任何 泛型参数)
fn foo<const N: usize>() {}

fn bar<T, const M: usize>() {
    foo::<M>(); // ok: 符合第一种
    foo::<2021>(); // ok: 符合第二种
    foo::<{20 * 100 + 20 * 10 + 1}>(); // ok: 符合第三种
    
    //foo::<{ M + 1 }>(); // error: 违背第三种，const 表达式中不能有泛型参数 M
    //foo::<{ std::mem::size_of::<T>() }>(); // error: 泛型表达式包含了泛型参数 T
    
    let _: [u8; M]; // ok: 符合第一种
    //let _: [u8; std::mem::size_of::<T>()]; // error: 泛型表达式包含了泛型参数 T
}

pub struct MinSlice<T, const N: usize> {
    pub head: [T; N],
    pub tail: [T],
}

fn main() {
    let slice: &[u8] = b"Hello, world";
    let reference: Option<&u8> = slice.get(6);
    // 我们知道 `.get` 返回的是 `Some(b' ')`
    // 但编译器不知道
    assert!(reference.is_some());

    let slice: &[u8] = b"Hello, world";

    // 当编译构建 MinSlice 时会进行长度检查，也就是在编译期我们就知道它的长度是 12
    // 在运行期，一旦 `unwrap` 成功，在 `MinSlice` 的作用域内，就再无需任何检查    
    let minslice = MinSlice::<u8, 12>::from_slice(slice).unwrap();
    let value: u8 = minslice.head[6];
    assert_eq!(value, b' ')
}

