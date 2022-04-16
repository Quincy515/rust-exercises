use std::fmt::format;

use aoko::{no_std::functions::ext::AnyExt, standard::functions::ext::StdAnyExt};

// 对 u8 类型做扩展，实现一个名为 u8_to_str 的关联函数，功能是 ****
trait A {
    fn u8_to_str(self) -> String;
}
impl A for u8 {
    fn u8_to_str(self) -> String {
        self.to_string()
    }
}

// 对（任意、所有）类型做扩展，实现一个名为 pipe 的关联函数，功能是将 self 作为闭包的参数，返回闭包执行的结果
trait B {
    fn pipe<F>(self, f: F) -> F::Output
    where
        F: FnOnce(Self) -> Self,
        Self: std::marker::Sized;
}

fn foo<T: A>(f: T) {}

fn bar<T>(f: T)
where
    T: A,
{
}

fn czz(f: impl A) {}

// 暂时未支持
// 1. 函数参数或返回值：impl Trait(impl)
// 2. 函数返回值：-> impl Trait() -> impl Trait()
// 3. trait 函数的返回值
// trait Err {
//     fn f(self) -> impl A;
// }

// fn col(f:impl A)-> impl Fn(impl A) -> String {
//     move |f| format!("hello {f}")

// }

fn su<T: A + std::fmt::Debug>(f: T) -> impl Fn(T) -> String {
    |f| format!("hello {f:?}")
}

fn main() {
    foo(0);
    // 0u8.type_name().sout();
    // String::new().type_name().sout();
    // (&String::new()).type_name().sout();
    // (&&String::new()).type_name().sout();
}
