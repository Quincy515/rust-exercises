fn main() {
    #[derive(Debug)]
    struct Trousers {
        code: u32,
        brand: String,
        len: u8,
    }

    impl Trousers {
        // 关联方法
        fn new(code: u32, brand: String, len: u8) -> Self {
            Trousers { code, brand, len }
        }

        // 实例方法
        fn set_len(&mut self, len: u8) {
            self.len = len;
        }
    }
    // 类型的实例化对象
    let mut trousers_a = Trousers {
        code: 1111,
        brand: String::from("A"),
        len: 10,
    };
    println!(
        "裤子: {}, 品牌: {}, 长度: {}",
        trousers_a.code, trousers_a.brand, trousers_a.len
    );
    trousers_a.set_len(9);
    println!(
        "裤子: {}, 品牌: {}, 长度: {}",
        trousers_a.code, trousers_a.brand, trousers_a.len
    );
    trousers_a.len = 8;
    println!(
        "裤子: {}, 品牌: {}, 长度: {}",
        trousers_a.code, trousers_a.brand, trousers_a.len
    );
    let trousers_b = Trousers::new(2222, String::from("B"), 11);
    println!(
        "裤子: {}, 品牌: {}, 长度: {}",
        trousers_b.code, trousers_b.brand, trousers_b.len
    );
    println!("裤子:{:#?}", trousers_b);
}
