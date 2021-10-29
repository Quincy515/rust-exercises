use std::{error::Error, process::Command};

/// BoxedError 类型，是 Box 的别名，它是 Error trait 的 trait object
/// 除了要求类型实现了 Error trait 外，它还有额外的约束：类型必须满足 Send / Sync 这两个 trait
pub type BoxedError = Box<dyn Error + Send + Sync>;

/// 构建了一个 Executor trait
pub trait Executor {
    fn run(&self) -> Result<Option<i32>, BoxedError>;
}

pub struct Shell<'a, 'b> {
    cmd: &'a str,
    args: &'b [&'a str],
}

impl<'a, 'b> Shell<'a, 'b> {
    pub fn new(cmd: &'a str, args: &'b [&'a str]) -> Self {
        Self { cmd, args }
    }
}

impl<'a, 'b> Executor for Shell<'a, 'b> {
    fn run(&self) -> Result<Option<i32>, BoxedError> {
        let output = Command::new(self.cmd).args(self.args).output()?;
        Ok(output.status.code())
        // 为了不让实现本身干扰接口调用的速度，我们在 trait 的方法中什么也不做，直接返回
        // Ok(Some(0))
    }
}

/// 使用泛型参数 impl Executor 使用的是泛型参数的简化版本 分配在栈上
pub fn execute_generics(cmd: &impl Executor) -> Result<Option<i32>, BoxedError> {
    cmd.run()
}

/// 使用 trait object: &dyn T 分配在堆上
pub fn execute_trait_object(cmd: &dyn Executor) -> Result<Option<i32>, BoxedError> {
    cmd.run()
}

/// 使用 trait object: Box<dyn T> 分配在堆上
pub fn execute_box_trait_object(cmd:Box<dyn Executor>) -> Result<Option<i32>,BoxedError> {
    cmd.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shell_shall_work() {
        let cmd = Shell::new("ls", &[]);
        let result = cmd.run().unwrap();
        assert_eq!(result, Some(0));
    }

    #[test]
    fn execute_shall_work() {
        let cmd = Shell::new("ls", &[]);

        let result = execute_generics(&cmd).unwrap();
        assert_eq!(result, Some(0));
        let result = execute_trait_object(&cmd).unwrap();
        assert_eq!(result, Some(0));
        let boxed = Box::new(cmd);
        let result = execute_box_trait_object(boxed).unwrap();
        assert_eq!(result, Some(0))
    }
}