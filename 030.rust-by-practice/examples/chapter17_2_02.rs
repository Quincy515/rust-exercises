// 2、 🌟🌟🌟🌟 使用 Box::leak 也可以产生 'static 生命周期
#[derive(Debug)]
struct Config {
    a: String,
    b: String,
}
static mut config: Option<&mut Config> = None;

/* 让代码工作，但不要修改函数的签名 */
fn init() -> Option<&'static mut Config> {
    // Some(&mut Config {
    //     a: "A".to_string(),
    //     b: "B".to_string(),
    // })
    let c = Box::new(Config {
        a: "A".to_string(),
        b: "B".to_string(),
    });
    Some(Box::leak(c))
}

fn main() {
    unsafe {
        config = init();

        println!("{:?}", config)
    }
}
