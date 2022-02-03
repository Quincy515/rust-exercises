fn base_converter(mut dec_num: u32, base: u32) -> String {
    // digits 对应各种余数的字符形式，尤其是 10 - 15
    let digits = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];
    let mut rem_stack = Stack::new();

    // 余数入栈
    while dec_num > 0 {
        let rem = dec_num % base;
        rem_stack.push(rem);
        dec_num /= base;
    }

    // 余数出栈并取出对应字符来拼成字符串
    let mut base_str = "".to_string();
    while !rem_stack.is_empty() {
        let rem = rem_stack.pop().unwrap() as usize;
        base_str += &digits[rem].to_string();
    }

    base_str
}

fn main() {
    let bin_str: String = base_converter(10, 2);
    let base_str: String = base_converter(43, 16);
    println!("10 is b{bin_str}, 43 is x{base_str}");
}

#[derive(Debug)]
struct Stack<T> {
    top: usize,   // 栈顶
    data: Vec<T>, // 栈数据容器
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            top: 0,
            data: Vec::new(),
        }
    }

    fn push(&mut self, val: T) {
        self.data.push(val); // 数据保存在 Vec 末尾
        self.top += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            return None;
        }
        self.top -= 1; // 栈顶减 1 后再弹出数据
        self.data.pop()
    }

    fn is_empty(&self) -> bool {
        0 == self.top
    }
}
