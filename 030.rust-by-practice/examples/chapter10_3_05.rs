/// 使用特征作为函数参数
/// 除了使用具体类型来作为函数参数，
/// 我们还能通过 impl Trait 的方式来指定实现了该特征的参数：
/// 该参数能接受的类型必须要实现指定的特征。


// 实现 `fn summary` 
// 修复错误且不要移除任何代码行
trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
struct Post {
    title: String,
    author: String,
    content: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("The author of post {} is {}", self.title, self.author)
    }
}

#[derive(Debug)]
struct Weibo {
    username: String,
    content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{} published a weibo {}", self.username, self.content)
    }
}

fn main() {
    let post = Post {
        title: "Popular Rust".to_string(),
        author: "Sunface".to_string(),
        content: "Rust is awesome!".to_string(),
    };
    let weibo = Weibo {
        username: "sunface".to_string(),
        content: "Weibo seems to be worse than Tweet".to_string(),
    };

    summary(&post);
    summary(&weibo);

    println!("{:?}", post);
    println!("{:?}", weibo);
}

// 在下面实现 `fn summary` 函数

fn summary(x: &impl Summary) {
    x.summarize();
}
