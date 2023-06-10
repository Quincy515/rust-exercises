# 线性代数

https://docs.rs/ndarray/latest/ndarray/doc/ndarray_for_numpy_users/index.html

## 1. 标量，向量，矩阵，张量

```rust
#extern crate ndarray;
#use ndarray::prelude::*;
#fn main() {
    // 标量
    let s = 5;

    // 向量
    let v = array!(1, 2);

    // 矩阵
    let m = array!([1, 2]);

    // 张量
    let t = array!(
        [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        [[11, 12, 13], [14, 15, 16], [17, 18, 19]],
        [[21, 22, 23], [24, 25, 26], [27, 28, 29]],
    );

    println!("标量：{s}");
    println!("向量：{v}");
    println!("矩阵：{m}");
    println!("张量：{t}");
#}
```
## 2. 矩阵转置

```rust
#extern crate ndarray;
#use ndarray::prelude::*;
#fn main() {
    // 矩阵转置
    let a = array!([1.0, 2.0], [1.0, 0.0], [2.0, 3.0]);
    let a_t = a.t();
    println!("A: {a}");
    println!("A 的转置：{a_t}");
#}
```

## 3. 矩阵加法

```rust
#extern crate ndarray;
#use ndarray::prelude::*;
#fn main() {
    // 矩阵加法
    let a = array!([1, 2], [3, 4]);
    let b = array!([6, 7], [8, 9]);
    println!("矩阵相加：{}", a + b);
#}
```

## 4. 矩阵乘法

```rust
#extern crate ndarray;
#use ndarray::prelude::*;
#fn main() {
    let m1 = array!([1,3],[1,0]);
    let m2 = array!([1,2],[5,0]);
    println!("按矩阵乘法规则：{}", m1.dot(&m2));
    println!("按逐元素相乘：{}", m1*m2);
    let v1 = array!(1,2);
    let v2 = array!(4,5);
    println!("向量内积：{}", v1.dot(&v2));
#}
```

## 5. 单位矩阵
为了引⼊矩阵的逆，我们需要先定义单位矩阵 (Identity Matrix)：单位矩阵乘以任意⼀个向量等于这个向量本⾝。

单位矩阵的结构⼗分简单，所有的对⾓元素都为 1 ，其他元素都为 0。

```rust
#extern crate ndarray;
#use ndarray::prelude::*;
#fn main() {
    // 声明一个单位矩阵
    let i:Array2<f64> = Array::eye(3);
    println!("单位矩阵：\n{}", i);
#}
```

## 6 矩阵的逆
```shel
cargo add ndarray-linalg
```

```rust
#extern crate ndarray;
#extern crate ndarray_linalg;
#use ndarray::prelude::*;
#use ndarray_linalg::Inverse;
#fn main() {
    let a: Array2<f64> = arr2(&[[1.0, 2.0], [3.0, 4.0]]);
    let a_inv = a.inv().unwrap();
    println!("矩阵的逆：\n{}", a_inv);
#}
```