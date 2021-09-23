mod opcode;

use opcode::{Code, Opcode};
use std::io::{Read, Write};

struct Interpreter {
    stack: Vec<u8>, // 表示无限长的纸带
}

impl Interpreter {
    fn new() -> Self {
        Self {
            stack: vec![0; 1], // 初始化长度是 1 默认值为 0
        }
    }

    fn run(&mut self, data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let code = Code::from(data)?;
        let code_len = code.instrs.len();
        let mut pc = 0; // Program counter 表示目前代码执行到的位置
        let mut sp = 0; // Stack Pointer 指针指向无限长的纸带的位置

        // 解释器主循环
        loop {
            if pc >= code_len {
                break; // 代码执行结束
            }
            // 获取指令
            match code.instrs[pc] {
                Opcode::SHR => {
                    sp += 1;
                    if sp == self.stack.len() {
                        self.stack.push(0);
                    }
                }
                Opcode::SHL => {
                    if sp != 0 {
                        sp -= 1;
                    }
                }
                Opcode::ADD => self.stack[sp] = self.stack[sp].overflowing_add(1).0, // 允许溢出
                Opcode::SUB => self.stack[sp] = self.stack[sp].overflowing_sub(1).0, // 允许溢出
                Opcode::PUTCHAR => std::io::stdout().write_all(&[self.stack[sp]])?,
                Opcode::GETCHAR => {
                    let mut buf: Vec<u8> = vec![0; 1];
                    std::io::stdin().read_exact(&mut buf)?;
                    self.stack[sp] = buf[0];
                }
                Opcode::LB => {
                    if self.stack[sp] == 0x00 {
                        pc = code.jtable[&pc];
                    }
                }
                Opcode::RB => {
                    if self.stack[sp] != 0x00 {
                        pc = code.jtable[&pc];
                    }
                }
            }
            // 当前指令结束，执行下一个指令
            pc += 1;
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let data = std::fs::read(&args[1])?;

    // let code = Code::from(data)?;
    // println!("{:?}", code.instrs);

    // 创建解释器
    let mut interpreter = Interpreter::new();
    interpreter.run(data);

    Ok(())
}
