// pipe
// 任意类型，都有关联函数 pipe，它将接收者传入用户自定义的闭包，作为第一个参数，返回闭包执行的结果

trait A<R> {
    fn pipe(self, f: impl FnOnce(Self) -> R) -> R
    where
        Self: Sized,
    {
        f(self)
    }

    fn pipe_ref<'a>(&'a self, f: impl FnOnce(&'a Self) -> R) -> R {
        f(self)
    }

    fn pipe_mut<'a>(&'a mut self, f: impl FnOnce(&'a mut Self) -> R) -> R {
        f(self)
    }
}

impl<T, R> A<R> for T {}

fn main() {
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
