fn main() {
    // 自定义品牌名称可用值枚举
    #[derive(Debug)]
    enum Brand {
        A(String),
        B,
    }
    #[derive(Debug)]
    struct Trousers {
        code: u32,
        brand: Brand,
        len: u8,
    }

    impl Trousers {
        // 关联方法
        fn new(code: u32, brand: Brand, len: u8) -> Self {
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
        brand: Brand::A(String::from("A1")),
        len: 10,
    };
    println!("裤子:{:#?}", trousers_a);
    trousers_a.set_len(9);
    println!("裤子:{:#?}", trousers_a);
    trousers_a.len = 8;
    println!("裤子:{:#?}", trousers_a);
    let trousers_b = Trousers::new(2222, Brand::B, 11);
    println!("裤子:{:#?}", trousers_b);
    match trousers_b.brand {
        Brand::B => println!("这是B品牌的裤子"),
        _ => println!("这是其他品牌的裤子"),
    }
}
