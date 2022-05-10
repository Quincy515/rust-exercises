// 方法
// 方法的生命周期标注跟函数类似。

// 示例

struct Owner(i32);

impl Owner {
    fn add_one<'a>(&'a mut self) { self.0 += 1; }
    fn print<'a>(&'a self) {
        println!("`print`: {}", self.0);
    }
}

fn demo() {
    let mut owner = Owner(18);

    owner.add_one();
    owner.print();
}


/* 添加合适的生命周期让下面代码工作 */
struct ImportantExcerpt<'a>{
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&'a self) -> i32 {
        3
    }
}

fn main() {
    demo()
}
