// 1. 定义一个上衣模块 coat
pub mod coat {
    // 2. 定义类型 Shirt
    #[derive(Debug)]
    pub struct Shirt {
        len: u8
    }
    impl Shirt {
        /// ```rust
        /// use rust_learn::coat::Shirt;
        /// use rust_learn::mother_change_len;
        /// let mut shirt = Shirt::new(10);
        /// mother_change_len(&mut shirt, 8);
        /// assert_eq!(shirt.get_len(), 8);
        /// ```
        pub fn new(len: u8) -> Self {
            Self { len }
        }
        pub fn get_len(&self) -> u8 {
            self.len
        }
    }
    impl super::SetLenable for Shirt {
        fn set_len(&mut self, len: u8) {
            self.len = len
        }
        fn change_able(&self) { // 方法重写
            println!("Shirt 的长度是可变的");
        }
    }

}

pub mod trousers {
    // 自定义品牌名称可用值枚举
    #[derive(Debug)]
    pub enum Brand {
        A(String),
        B,
        C
    }
    #[derive(Debug)]
    pub struct Trousers {
        pub code: u32,
        pub brand: Brand,
        len: u8,
        create_date: String,
    }

    impl super::SetLenable for Trousers {
        // 实例方法
        fn set_len(&mut self, len: u8) {
            if len >= 8 && len <= 10 {
                self.len = len;
            }
        }
    }
    impl Trousers {
        // 关联方法
        pub fn new(code: u32, brand: Brand, len: u8) -> Self {
            Trousers {
                code,
                brand,
                len,
                create_date: String::from("2022-2-22"),
            }
        }
        pub fn get_len(&self) -> u8 {
            self.len
        }
    }
}
pub trait SetLenable {
    fn set_len(&mut self, len: u8);
    fn change_able(&self) { // 给特征添加默认方法,可以继承
        println!("长度是可变的");
    }
}

// pub fn mother_change_len(set_len_able: &mut impl SetLenable, len: u8) {
// pub fn mother_change_len<T: SetLenable>(set_len_able: &mut T, len: u8) {
pub fn mother_change_len<T>(set_len_able: &mut T, len: u8)
where
    T: SetLenable,
{
    set_len_able.set_len(len);
}

#[cfg(test)]
mod tests {
    #[test]
    fn mother_change_len_test() {
        let mut shirt_a = super::coat::Shirt::new(10);
        super::mother_change_len(&mut shirt_a, 8);
        assert_eq!(shirt_a.get_len(), 8);
    }
}