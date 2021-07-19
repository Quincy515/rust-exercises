use rust_learn::trousers::*;

fn create_box_of_trousers() -> Vec<Trousers> {
    let trousers_a1 = Trousers::new(111, Brand::A(String::from("A1")), 9);
    let trousers_b1 = Trousers::new(221, Brand::B, 10);
    let trousers_b2 = Trousers::new(222, Brand::B, 9);
    let trousers_c = Trousers::new(331, Brand::C, 8);

    vec![trousers_a1, trousers_b1, trousers_b2, trousers_c]
}
#[test]
fn find_trousers_by_brand() {
    let trousers_box = create_box_of_trousers();

    // 1. 命令式 使用代码描述怎么做 How to do
    let mut brand_b_trousers_box = vec![];
    for trousers in trousers_box.iter() {
        if let Brand::B = trousers.brand {
            brand_b_trousers_box.push(trousers);
        }
    }

    for trousers in brand_b_trousers_box.iter() {
        println!("trousers:{:?}", trousers);
    }
    // 2. 声明式 使用代码描述做什么 What to do
    let brand_b_trousers_box2: Vec<&Trousers> = trousers_box
        .iter()
        .filter(|trousers| {
            if let Brand::B = trousers.brand {
                true
            } else {
                false
            }
        })
        .collect();
    brand_b_trousers_box2
        .iter()
        .for_each(|trousers| println!("trousers 2:{:?}", trousers));

    // 函数式编程尽量编写纯函数 有参数和返回值，声明式代码
}

fn find_trousers_by_len(trousers_box: &Vec<Trousers>, len: u8) -> Vec<&Trousers> {
    trousers_box
        .iter()
        .filter(|trousers| {
            if trousers.get_len() == len {
                true
            } else {
                false
            }
        })
        .collect()
}

fn print_each_trousers(trousers_box: &Vec<&Trousers>) {
    trousers_box
        .iter()
        .for_each(|trousers| println!("trousers:{:?}", trousers));
}

#[test]
fn find_trousers_by_len_test() {
    let trousers_box = create_box_of_trousers();
    let find_trousers = find_trousers_by_len(&trousers_box, 8);
    print_each_trousers(&find_trousers);
}

fn find_trousers_by_fn(
    trousers_box: &Vec<Trousers>,
    condition: impl FnMut(&&Trousers) -> bool,
) -> Vec<&Trousers> {
    trousers_box.iter().filter(condition).collect()
}

fn find_len_gt_8_condition(trousers: &&Trousers) -> bool {
    if trousers.get_len() > 8 {
        true
    } else {
        false
    }
}
#[test]
fn find_trousers_by_fn_test() {
    let trousers_box = create_box_of_trousers();
    let find_trousers = find_trousers_by_fn(&trousers_box, find_len_gt_8_condition);
    print_each_trousers(&find_trousers);
    println!("闭包");
    let mut len = 9;
    let find_trousers_lt_10 = find_trousers_by_fn(&trousers_box, move |trousers| {
        if trousers.get_len() < len {
            len = len + 1;
            println!("len in closure:{}", len);
            true
        } else {
            false
        }
    });
    println!("len:{}", len);
    print_each_trousers(&find_trousers_lt_10);
}
