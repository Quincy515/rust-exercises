fn main() {
    use rust_learn::trousers::Brand;
    use rust_learn::trousers::Trousers;
    use rust_learn::SetLenable;
    // 类型的实例化对象
    let mut trousers_a = Trousers::new(1111, Brand::A(String::from("A1")), 10);
    println!("裤子:{:#?}", trousers_a);
    trousers_a.set_len(9);
    println!("裤子:{:#?}", trousers_a);
    // trousers_a.len = 8;
    // println!("裤子:{:#?}", trousers_a);
    let trousers_b = Trousers::new(2222, Brand::B, 11);
    println!("裤子:{:#?}", trousers_b);
    match trousers_b.brand {
        Brand::B => println!("这是B品牌的裤子"),
        _ => println!("这是其他品牌的裤子"),
    }
    if let Brand::B = trousers_b.brand {
        println!("这是B品牌的裤子 if let");
    }
    rust_learn::mother_change_len(&mut trousers_a, 10);
    println!("裤子:{:#?}", trousers_a);

    use rust_learn::coat::Shirt;
    let mut shirt_a = Shirt::new(10);
    println!("上衣:{:?}", shirt_a);
    rust_learn::mother_change_len(&mut shirt_a, 8);
    println!("上衣:{:?}", shirt_a);
    shirt_a.change_able();

}
