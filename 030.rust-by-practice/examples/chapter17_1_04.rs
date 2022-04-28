// 4、🌟🌟🌟

/* 使用三种方法修复下面的错误  */
fn invalid_output_1<'a>() -> String {
    String::from("foo")
}

fn invalid_output_2() -> &'static str {
    "foo"
}

fn invalid_output_3<'a>(s: &'a String) -> &'a String {
    s
}

fn main() {}
