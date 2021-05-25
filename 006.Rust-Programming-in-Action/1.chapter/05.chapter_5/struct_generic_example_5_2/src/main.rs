struct Rectangle1<T> {
    width: T,
    height: T,
}

struct Rectangle2<T, U> {
    width: T,
    height: U,
}

// 实现泛型方法
// 带有泛型类型的参数或返回值的方法叫做泛型方法。
// 要定义泛型结构体 Rectangle1<T> 的泛型方法，
// 需要在 impl 后面带上 <T>,
// 而方法的参数或返回值的参数才能使用 T。
impl<T> Rectangle1<T> {
    fn width(&self) -> &T {
        &self.width
    }
    fn height(&self) -> &T {
        &self.height
    }
}

// 为指定的类型实现方法，Rectangle1<i32> 实现了 area 方法，其他非 i32 类型的实例没有 area 方法
impl Rectangle1<i32> {
    fn area(&self) -> i32 {
        self.width * self.height
    }
}

impl<T, U> Rectangle2<T, U> {
    fn width(&self) -> &T {
        &self.width
    }
    fn height(&self) -> &U {
        &self.height
    }
}
fn main() {
    // let rect1 = Rectangle1 { width: 8, height: 2.2 };
    let rect1 = Rectangle1 {
        width: 8,
        height: 2,
    };
    println!("rect1.width:{},rect1.height:{}", rect1.width, rect1.height);
    println!("rect1.area:{}", rect1.area());
}
