fn num2str_stk(mut num: i32, base: i32) -> String {
    let digits: [&str; 16] = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F",
    ];
    let mut rem_stack = Stack::new();

    while num > 0 {
        if num < base {
            rem_stack.push(num); // 不超过 base 直接入栈
        } else {
            // 超过 base 余数入栈
            rem_stack.push(num % base);
        }
        num /= base;
    }

    // 出栈余数并组成字符串
    let mut numstr = "".to_string();
    while !rem_stack.is_empty() {
        numstr += digits[rem_stack.pop().unwrap() as usize];
    }

    numstr
}

fn main() {
    let num = 100;
    let sb = num2str_stk(num, 2);
    let so = num2str_stk(num, 8);
    let sh = num2str_stk(num, 16);
    println!("{num} is b{sb}, o{so}, x{sh}");
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
