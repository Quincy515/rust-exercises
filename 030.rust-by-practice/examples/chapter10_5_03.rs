/// 完全限定语法
/// 在 Rust 中，两个不同特征的方法完全可以同名，
/// 且你可以为同一个类型同时实现这两个特征。
/// 这种情况下，就出现了一个问题：
/// 该如何调用这两个特征上定义的同名方法。
/// 为了解决这个问题，我们需要使用完全限定语法( Fully Qualified Syntax )。

/// 示例
/*
trait UsernameWidget {
    fn get(&self) -> String;
}

trait AgeWidget {
    fn get(&self) -> u8;
}

struct Form {
    username: String,
    age: u8,
}

impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}

impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

fn main() {
    let form = Form{
        username: "rustacean".to_owned(),
        age: 28,
    };

    // 如果你反注释下面一行代码，将看到一个错误: Fully Qualified Syntax
    // 毕竟，这里有好几个同名的 `get` 方法
    // 
    // println!("{}", form.get());
    
    let username = UsernameWidget::get(&form);
    assert_eq!("rustacean".to_owned(), username);
    let age = AgeWidget::get(&form); // 你还可以使用以下语法 `<Form as AgeWidget>::get`
    assert_eq!(28, age);

    println!("Success!")
}

*/

trait Pilot {
    fn fly(&self) -> String;
}

trait Wizard {
    fn fly(&self) -> String;
}

struct Human;

impl Pilot for Human {
    fn fly(&self) -> String {
        String::from("This is your captain speaking.")
    }
}

impl Wizard for Human {
    fn fly(&self) -> String {
        String::from("Up!")
    }
}

impl Human {
    fn fly(&self) -> String {
        String::from("*waving arms furiously*")
    }
}

fn main() {
    let person = Human;

    assert_eq!(Pilot::fly(&person), "This is your captain speaking.");
    assert_eq!(Wizard::fly(&person), "Up!");

    assert_eq!(person.fly(), "*waving arms furiously*");

    println!("Success!")
}

