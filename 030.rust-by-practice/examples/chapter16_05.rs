/// 5.🌟🌟 我们还能使用 0 来填充数字
fn main() {
    println!("Hello {:5}!", 5); // => Hello     5!
    println!("Hello {:+}!", 5); // =>  Hello +5!
    println!("Hello {:05}!", 5); // => Hello 00005!
    println!("Hello {:05}!", -5); // => Hello -0005!

    /* 填空 */
    assert!(format!("{number:0>width$}", number=1, width=6) == "000001");
    
    println!("Success!")
}

