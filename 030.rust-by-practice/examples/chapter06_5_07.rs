/// 打印结构体
/// 7. 🌟🌟 我们可以使用 #[derive(Debug)] 让结构体变成可打印的.


// 填空，让代码工作
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // 打印 debug 信息到标准错误输出 stderr,并将 `30 * scale` 的值赋给 `width`
        height: 50,
    };

    dbg!(&rect1); // 打印 debug 信息到标准错误输出 stderr

    println!("{:?}", rect1); // 打印 debug 信息到标准输出 stdout
}

