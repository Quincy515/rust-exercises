fn pal_checker(pal: &str) -> bool {
    let mut d = Deque::new(pal.len());
    for c in pal.chars() {
        let _r = d.add_rear(c);
    }
    let mut is_pal = true;
    while d.size() > 1 && is_pal {
        let head = d.remove_front();
        let tail = d.remove_rear();
        // 比较首尾字符，若不同则非回文
        if head != tail {
            is_pal = false;
        }
    }
    is_pal
}

fn main() {
    let pal = "rustsur";
    let is_pal = pal_checker(pal);
    println!("{pal} is palindrome string: {is_pal}");
}

/// 双端队列
#[derive(Debug)]
struct Deque<T> {
    cap: usize,   // 容量
    data: Vec<T>, // 数据容器
}

impl<T> Deque<T> {
    fn new(cap: usize) -> Self {
        Deque {
            cap: cap,
            data: Vec::with_capacity(cap),
        }
    }

    // Vec 末尾为队首
    fn add_front(&mut self, val: T) -> Result<(), String> {
        if Self::size(&self) == self.cap {
            return Err("No space available".to_string());
        }
        self.data.push(val);

        Ok(())
    }

    // Vec 首部为队尾
    fn add_rear(&mut self, val: T) -> Result<(), String> {
        if Self::size(&self) == self.cap {
            return Err("No space available".to_string());
        }
        self.data.insert(0, val);

        Ok(())
    }

    // 从队首移除数据
    fn remove_front(&mut self) -> Option<T> {
        if Self::size(&self) > 0 {
            self.data.pop()
        } else {
            None
        }
    }

    // 从队尾移除数据
    fn remove_rear(&mut self) -> Option<T> {
        if Self::size(&self) > 0 {
            Some(self.data.remove(0))
        } else {
            None
        }
    }

    fn is_empty(&self) -> bool {
        0 == Self::size(&self)
    }

    fn size(&self) -> usize {
        self.data.len()
    }
}
