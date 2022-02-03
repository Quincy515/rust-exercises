fn hot_potato(names: Vec<&str>, num: usize) -> &str {
    // 初始化队列，名字入队
    let mut q = Queue::new(names.len());
    for name in names {
        let _rm = q.enqueue(name);
    }

    while q.size() > 1 {
        // 出入栈名字，相当于传递山芋
        for _i in 0..num {
            let name = q.dequeue().unwrap();
            let _rm = q.enqueue(name);
        }

        // 出入栈达到 num 次，删除一人
        let _rm = q.dequeue();
    }
    q.dequeue().unwrap()
}

fn main() {
    let name = vec!["Shieber", "David", "Susan", "Jane", "Kew", "Brad"];
    let rem = hot_potato(name, 8);
    println!("The left person is {}", rem);
}

/// 队列定义
#[derive(Debug)]
struct Queue<T> {
    cap: usize,   // 容量
    data: Vec<T>, // 数据容器
}

impl<T> Queue<T> {
    fn new(size: usize) -> Self {
        Queue {
            data: Vec::with_capacity(size),
            cap: size,
        }
    }

    // 判断是否有剩余空间，有则数据加入队列
    fn enqueue(&mut self, val: T) -> Result<(), String> {
        if Self::size(&self) == self.cap {
            return Err("No space available".to_string());
        }
        self.data.insert(0, val);

        Ok(())
    }

    // 数据出队
    fn dequeue(&mut self) -> Option<T> {
        if Self::size(&self) > 0 {
            self.data.pop()
        } else {
            None
        }
    }

    fn size(&self) -> usize {
        self.data.len()
    }
}
