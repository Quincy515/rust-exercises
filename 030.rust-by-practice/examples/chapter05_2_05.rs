
fn main() {
    let mut s = String::from("hello, ");

    // 填写空白处，让代码工作
    let mut p = s.clone();
    
    p.push_str("world");

    let p = &mut s;
    p.push_str("你好");
}

