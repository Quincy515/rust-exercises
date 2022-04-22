/// 使用 pub use 进行再导出
/// 3. 🌟🌟🌟 在之前创建的hello-package 的库包中, 
/// 添加一些代码让下面的代码能够正常工作

pub use crate::front_of_house::hosting;


fn main() {
    assert_eq!(hello_package::hosting::seat_at_table(), "sit down please");
     assert_eq!(hello_package::eat_at_restaurant(),"yummy yummy!");
}

