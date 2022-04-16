// pipe
// 任意类型，都有关联函数 pipe，它将接收者传入用户自定义的闭包，作为第一个参数，返回闭包执行的结果

use std::io::stdin;

use aoko::standard::functions::ext::StdAnyExt;

trait A<R> {
    fn pipe(self, f: impl FnOnce(Self) -> R) -> R
    where
        Self: Sized,
    {
        f(self)
    }

    fn tap(self, f: impl FnOnce(&Self) -> R) -> Self
    where
        Self: Sized,
    {
        f(&self);
        self
    }
    fn tap_mut(mut self, f: impl FnOnce(&mut Self) -> R) -> Self
    where
        Self: Sized,
    {
        f(&mut self);
        self
    }

    // fn tap(self, f: impl FnOnce(&Self)) -> Self
    // where
    //     Self: Sized,
    // {
    //     f(&self);
    //     self
    // }
    // fn tap_mut(mut self, f: impl FnOnce(&mut Self)) -> Self
    // where
    //     Self: Sized,
    // {
    //     f(&mut self);
    //     self
    // }

    fn pipe_ref<'a>(&'a self, f: impl FnOnce(&'a Self) -> R) -> R {
        f(self)
    }

    fn pipe_mut<'a>(&'a mut self, f: impl FnOnce(&'a mut Self) -> R) -> R {
        f(self)
    }
}

impl<T, R> A<R> for T {}

fn main() {
    [1, 2, 3].to_vec().tap_mut(|arr| arr.push(4)).sout();

    "abc".pipe(|s| println!("{s}"));
    String::pipe("abc".to_owned(), |s| println!("{s}"));
    "abc".to_owned().pipe_mut(|s| {
        *s += "abc";
        println!("{s}")
    });
    let res = [1, 2, 3].pipe_ref(|arr| arr[0]);
    println!("{res}");
    let res = [1, 2, 3].pipe_ref(|arr| &arr[0]);
    println!("{res}");
}

pub fn read_line1() -> String {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    s
}

pub fn read_line2() -> String {
    // 不符合人体工程学，返回单元值，但不会忽略错误
    String::new().tap_mut(|s| {
        stdin().read_line(s).unwrap();
    })
}

pub fn read_line3() -> String {
    // 返回泛型隐藏的问题是会把error抛出去，忽略了错误处理
    String::new().tap_mut(|s| stdin().read_line(s) /*.unwrap() */)
}
