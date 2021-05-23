fn main() {
    // 1. &str 字符串的创建
    // 固定长度的字符串字面量 str，通常以引用的形式 &str 出现。
    // 字符串字面量 &str 是字符的集合，代表的是不可变的 UTF-8 编码的字符串的引用，
    // 创建后无法再为其追加内容或更改内容
    // 使用双引号创建字符串字面量
    let s1 = "Hello, Rust!";
    // 使用 as_str 方法将字符串对象转换为字符串字面量
    let str = String::from("Hello, Rust!");
    let s2 = str.as_str();

    // 2. String 的创建
    // String 类型的本质是一个字段为 Vec<u8> 类型的结构体，
    // 它把字符内容存放在堆上，由指向堆上字节序列的指针 (as_ptr方法)
    // 记录堆上字节序列的长度 (len方法)
    // 和堆分配的容量 (capacity方法) 3 部分组成
    // 使用 String::new 函数创建空的字符串对象
    let mut s = String::new();
    // 使用 String::from 函数根据指定的字符串字面量创建字符串对象
    let s = String::from("Hello, Rust!");
    // 使用 to_string 方法将字符串字面值转换为字符串对象
    let str = "Hello, Rust!";
    let s = str.to_string();

    // 3. 字符串的修改
    let mut s = String::from("Hello, ");
    s.push('R'); // push 方法在字符串后追加字符
    s.push_str("ust!"); // push_str 方法在字符串后最佳字符串字面量

    println!("{}", s); // Hello, Rust!

    let mut s = String::from("Hello World!");
    s.insert(5, ','); // 使用 insert 方法在字符串中插入字符
    s.insert_str(7, "Rust "); // 使用 insert_str 方法在字符串中插入字符串字面量

    // 使用"+" 或 "+=" 运算符将两个字符串连接成一个新的字符串，
    // 要求运算符右边必须是字符串字面量，不能对两个 String 类型的字符串使用
    let s1 = String::from("Hello");
    let s2 = String::from(", ");
    let s3 = String::from("Rust ");
    let s4 = "World";
    let mut s = s1 + &s2 + s3.as_str() + s4;
    s += "!";

    println!("{}", s); // Hello, Rust World!

    // 使用 fromat! 连接字符串,对于 String 类型和 &str 类型的字符串都适用
    let s1 = String::from("Hello");
    let s2 = String::from("Rust");
    let s3 = "World";
    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("{}", s); // Hello-Rust-World

    // 使用 replace 和 replacen 方法将字符串中指定的子串替换为另一个字符串。
    let s = String::from("aaabbbbccaadd");
    let s1 = s.replace("aa", "77"); // 第1个参数是要被替换的字符串子串，第2个参数是新字符串
    let s2 = s.replacen("aa", "77", 1); // 第2个参数指定替换的个数

    println!("{}", s1); // 77abbbbcc77dd
    println!("{}", s2); // 77abbbbccaadd
}
