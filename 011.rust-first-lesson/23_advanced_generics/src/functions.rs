pub trait ImplTrait {
    // 允许
    fn impl_in_args(s: impl Into<String>) -> String {
        s.into()
    }

    // 不允许 Rust 目前还不支持在 trait 里使用 impl trait 做返回值
    // fn impl_as_return(s:String)-> impl Into<String> {
    //     s
    // }
}


// 可以正确编译
pub fn generics_as_return_working(i: u32) -> impl Iterator<Item=u32> {
    std::iter::once(i)
}

// 期待泛型参数，却返回一个具体类型
// 使用泛型参数做返回值，在实现的时候会很麻烦，很难在函数中正确构造一个返回泛型参数的语句
// pub fn generics_as_return_not_working<T: Iterator<Item=u32>>(i: u32) -> T {
//     std::iter::once(i)
// }

// 返回 trait object
// 它消除了类型的差异，把所有不同的实现 Iterator 的类型都统一到一个相同的 trait object 下
pub fn trait_object_as_return_working(i: u32) -> Box<dyn Iterator<Item=u32>> {
    Box::new(std::iter::once(i))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_return_impl() {
        let mut iter = generics_as_return_working(10);
        assert_eq!(Some(10), iter.next());
    }

    #[test]
    fn test_return_trait_object() {
        let mut iter = trait_object_as_return_working(10);
        assert_eq!(Some(10), iter.next());
    }
}