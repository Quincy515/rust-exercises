/// 4. 🌟🌟 匹配守卫（match guard）
/// 是一个位于 match 分支模式之后的额外 if 条件，
/// 它能为分支模式提供更进一步的匹配条件。


// 填空让代码工作，必须使用 `split`
fn main() {
    let num = Some(4);
    let split = 5;
    match num {
        Some(x) if x < split => assert!(x < split),
        Some(x) => assert!(x >= split),
        None => (),
    }
}

