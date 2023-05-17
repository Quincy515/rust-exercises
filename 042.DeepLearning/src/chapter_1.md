# Chapter 1

https://docs.rs/ndarray/latest/ndarray/doc/ndarray_for_numpy_users/index.html



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


```rust
#extern crate ndarray;
#use ndarray::prelude::*;
#fn main() {

#}
```


```rust
#extern crate ndarray;
#use ndarray::prelude::*;
#fn main() {

#}
```


```rust
#extern crate ndarray;
#use ndarray::prelude::*;
#fn main() {

#}
```