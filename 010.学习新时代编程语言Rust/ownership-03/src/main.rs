use std::collections::HashMap;

fn main() {
    // 小胖的一箱裤子
    // 定义一个HashMap类型的箱子存放裤子
    let mut xiaopang_box_trousers = HashMap::new();
    // 使用品牌名称作为key，同品牌的裤子组成的Vector作为Value
    xiaopang_box_trousers.insert(
        "A",
        vec![String::from("1.##########"), String::from("2.#########")],
    );
    xiaopang_box_trousers.insert(
        "B",
        vec![String::from("3.########"), String::from("4.#######")],
    );
    xiaopang_box_trousers.insert("C", vec![String::from("5.######")]);

    let brand = "B"; // 查找的品牌名称
    println!("查找{}品牌的裤子", brand);
    let trousers_opt = xiaopang_box_trousers.get(&brand);
    match trousers_opt {
        Some(trouser_vec) => println!("找到了品牌为{}的裤子{:?}", brand, trouser_vec),
        None => println!("没有找到编号为{}的裤子", brand),
    }
}
