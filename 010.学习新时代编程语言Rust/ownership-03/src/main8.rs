use std::collections::HashMap;

fn main() {
    // 小胖的一箱裤子
    let mut xiaopang_box_trousers = HashMap::new();
    xiaopang_box_trousers.insert(1, String::from("1.##########"));
    xiaopang_box_trousers.insert(2, String::from("2.#########"));
    xiaopang_box_trousers.insert(3, String::from("3.########"));
    xiaopang_box_trousers.insert(4, String::from("4.#######"));
    xiaopang_box_trousers.insert(5, String::from("5.######"));

    let code = 2;
    println!("查找编号为{}的裤子", code);
    let trousers_opt = xiaopang_box_trousers.get(&code);
    match trousers_opt {
        Some(trouser) => println!("找到了编号为{}的裤子{}", code, trouser),
        None => println!("没有找到编号为{}的裤子", code),
    }
}
