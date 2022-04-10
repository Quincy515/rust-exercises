/**
Module
在 Rust 语言圣经中，我们已经深入讲解过模块module，这里就不再赘述，直接开始我们的练习。

之前我们创建了一个 package hello-package，它的目录结构在经过多次修改后，变成了以下模样:


.
├── Cargo.toml
├── src
│   ├── lib.rs
│   └── main.rs
下面，我们来为其中的库包创建一些模块，然后在二进制包中使用这些模块。

1. 🌟🌟 根据以下的模块树描述实现模块 front_of_house :

库包的根(src/lib.rs)
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         ├── take_payment
         └── complain
// 填空
// in lib.rs

mod front_of_house {
    // 实现此模块
    mod hosting{
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving{
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {} 
        fn complain() {}
    }
}

2. 🌟🌟 让我们在库包的根中定义一个函数 eat_at_restaurant, 
然后在该函数中调用之前创建的函数 eat_at_restaurant


#![allow(unused)]
fn main() {
// in lib.rs

// 填空并修复错误

// 提示：你需要通过 `pub` 将一些项标记为公有的，这样模块 `front_of_house` 中的项才能被模块外的项访问
mod front_of_house {
    /* ...snip... */
        pub mod hosting {
            pub fn add_to_waitlist() {}
            pub fn seat_at_table() {}
        }
        
    pub mod serving{
        pub fn take_order() {}
        pub fn serve_order() {}
        pub fn take_payment() {} 
        fn complain() {}
    }
}

pub fn eat_at_restaurant() {
    // 使用绝对路径调用
    crate::front_of_house::hosting::add_to_waitlist();

    // 使用相对路径调用
     front_of_house::add_to_waitlist();
}


3. 🌟🌟 我们还可以使用 super 来导入父模块中的项
// in lib.rs

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // 使用三种方式填空
        //1. 使用关键字 `super`
        //2. 使用绝对路径
        super::front_of_house::serving.serve_order();
        crate::front_of_house::serving.serve_order();
    }

    fn cook_order() {}
}


4. 🌟🌟🌟🌟 请将上面的模块和代码分离到以下目录文件中e :

.
├── Cargo.toml
├── src
│   ├── back_of_house.rs
│   ├── front_of_house
│   │   ├── hosting.rs
│   │   ├── mod.rs
│   │   └── serving.rs
│   ├── lib.rs
│   └── main.rs

// in src/lib.rs

mod front_of_house;
mod back_of_house;
pub fn eat_at_restaurant() -> String {
    front_of_house::hosting::add_to_waitlist();

    back_of_house::cook_order();

    String::from("yummy yummy!")
}

// in src/back_of_house.rs

use crate::front_of_house;
pub fn fix_incorrect_order() {
    cook_order();
    front_of_house::serving::serve_order();
}

pub fn cook_order() {}

// in src/front_of_house/mod.rs

pub mod hosting;
pub mod serving;

// in src/front_of_house/hosting.rs

pub fn add_to_waitlist() {}

pub fn seat_at_table() -> String {
    String::from("sit down please")
}

// in src/front_of_house/serving.rs

pub fn take_order() {}

pub fn serve_order() {}

pub fn take_payment() {}

// Maybe you don't want the guest hearing the your complaining about them
// So just make it private
fn complain() {}


5. 🌟🌟🌟现在我们可以从二进制包中发起函数调用了.
mod front_of_house;

fn main() {
    assert_eq!(front_of_house::hosting::seat_at_table(), "sit down please");
    assert_eq!(hello_package::eat_at_restaurant(),"yummy yummy!");
}
* /
