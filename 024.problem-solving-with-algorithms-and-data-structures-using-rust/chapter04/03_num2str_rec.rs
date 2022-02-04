const BASESTR: [&str; 16] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F",
];

/// 可以把递归看成是栈，只是这个栈是由编译器为我们隐式调用的，
/// 代码中只用了递归，但编译器使用了栈来保存数据
fn num2str_rec(num: i32, base: i32) -> String {
    if num < base {
        BASESTR[num as usize].to_string()
    } else {
        // 余数加在末尾
        num2str_rec(num / base, base) + BASESTR[(num % base) as usize]
    }
}

fn main() {
    let num = 100;
    let sb = num2str_rec(num, 2);
    let so = num2str_rec(num, 8);
    let sh = num2str_rec(num, 16);
    println!("{num} is b{sb}, o{so}, x{sh}");
}
