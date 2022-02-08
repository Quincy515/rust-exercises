fn par_checker1(par: &str) -> bool {
    let mut char_list = Vec::new();
    for c in par.chars() {
        char_list.push(c);
    }

    let mut index = 0;
    let mut balance = true; // 括号是否匹配（平衡）标示
    let mut stack = Stack::new(); // 使用前面实现的栈

    while index < char_list.len() && balance {
        let c = char_list[index];

        if '(' == c {
            // 如果为开符号，入栈
            stack.push(c);
        } else {
            // 如果为闭符号，判断栈是否为空
            if stack.is_empty() {
                balance = false; // 为空则不平衡
            } else {
                let _r = stack.pop();
            }
        }
        index += 1;
    }
    // 平衡且栈为空，括号表达式才是匹配的
    balance && stack.is_empty()
}

fn main() {
    let sa = "()(())";
    let sb = "()((()";
    let res1 = par_checker1(sa);
    let res2 = par_checker1(sb);
    println!("sa balanced: {res1}, sb balanced: {res2}");
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
