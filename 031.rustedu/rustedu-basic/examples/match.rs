#![allow(dead_code)]
// tagged union type 标签联合类型 X、Y、Z（携带或者不携带值）
enum A {
    X,
    Y(u8),         // 匿名元组
    Z { a: bool }, // 结构体
}

fn main() {
    // 全匹配
    let _foo = A::X;

    // 流程控制
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("{}", result);

    // 枚举
    use A::*;
    // match A {
    //     X => (),
    //     Y(x) => println!("{}", x),
    //     Z { a: foo } => println!("{}", foo),
    // }

    let a = [1, 2, 3, 4, 5];
    // match a {
    //     [foo, ..] => {
    //         dbg!(foo);
    //     }
    //     [foo, bar, ..] => { // unreachable_patterns 已经匹配过了
    //         dbg!(foo, bar);
    //     }
    // }
    let arr = a;
    let [a, ..] = arr;
    dbg!(a);

    let [a, b, ..] = arr;
    dbg!(a, b);

    let [.., last] = arr;
    dbg!(last);

    let [_, mid, ..] = arr;
    dbg!(mid);

    let [_, mid @ .., _] = arr;
    dbg!(mid);

    // 或模式
    let a = X;
    if let 0 | 1 = 128u8 {}

    match a {
        X | Y(0) => (),
        Z { a: _ } => (),
        _ => (),
    }

    // ref 引用
    match a {
        X => (),
        Y(ref x) => println!("{}", x),
        Z { a: ref x } => println!("{}", x),
    }

    let ref_foo = "".to_owned();
    match ref_foo {
        ref x => x,
        _ => "",
    };

    // 解构
    let foo = (0, true, vec![1, 2, 3]);
    let (a, b, c) = foo;

    // to_owned() to_string()
    let stri = "".to_string(); // 更通用 display trait
    let own = "".to_owned(); // &str -> String 获取所有权
}

fn string_to_owned() -> String {
    let s = String::new();
    s.to_owned()
}

// 静态提升
fn static_promotion() -> &'static str {
    let a = "";
    a
}

