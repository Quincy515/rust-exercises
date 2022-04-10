/// 4.🌟🌟🌟 左对齐, 右对齐, 使用指定的字符填充

fn main() {
    // 左对齐
    println!("Hello {:<5}!", "x"); // => Hello x    !
    // 右对齐
    assert_eq!(format!("Hello {:>5}!", "x"), "Hello     x!");
    // 居中对齐
    assert_eq!(format!("Hello {:^5}!", "x"), "Hello   x  !");

    // 左对齐，并使用 `&` 填充
    assert_eq!(format!("Hello {:&<5}!", "x"), "Hello x&&&&!");

    println!("Success!")
}

