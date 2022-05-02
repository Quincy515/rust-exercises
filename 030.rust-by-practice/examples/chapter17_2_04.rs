// 4、 &'static 可以被强转成一个较短的生命周期

// Example


// 声明一个 static 常量，它拥有 `'static` 生命周期.
static NUM: i32 = 18;

// 返回常量 `Num` 的引用，注意，这里的生命周期从 `'static` 强转为 `'a`
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        let lifetime_num = 9;

        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible!", NUM);
}
