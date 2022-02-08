use std::fmt::{Debug, Display};

/// 子节点链接
type Link<T> = Option<Box<BinaryTree<T>>>;

/// 二叉树定义
/// key 保存数据
/// left 和 right 保存左右子节点链接
#[derive(Debug, Clone)]
struct BinaryTree<T> {
    key: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T: Clone> BinaryTree<T> {
    fn new(key: T) -> Self {
        BinaryTree {
            key: key,
            left: None,
            right: None,
        }
    }

    // 新子节点作为根节点的左子节点
    fn insert_left_tree(&mut self, key: T) {
        if self.left.is_none() {
            let node = BinaryTree::new(key);
            self.left = Some(Box::new(node));
        } else {
            let mut node = BinaryTree::new(key);
            node.left = self.left.take();
            self.left = Some(Box::new(node));
        }
    }

    // 新子节点作为根节点的右子节点
    fn insert_right_tree(&mut self, key: T) {
        if self.right.is_none() {
            let node = BinaryTree::new(key);
            self.right = Some(Box::new(node));
        } else {
            let mut node = BinaryTree::new(key);
            node.right = self.right.take();
            self.right = Some(Box::new(node));
        }
    }

    // 获取左右子节点及根节点，注意使用了 clone
    fn get_left(&self) -> Link<T> {
        self.left.clone()
    }

    fn get_right(&self) -> Link<T> {
        self.right.clone()
    }

    fn get_key(&self) -> T {
        self.key.clone()
    }

    fn set_key(&mut self, key: T) {
        self.key = key;
    }
}

fn main() {
    let mut bt = BinaryTree::new('a');

    let root = bt.get_key();
    println!("root val is {:?}", root);

    let left = bt.get_left();
    println!("left val is {:#?}", left);

    let right = bt.get_right();
    println!("right val is {:#?}", right);

    bt.insert_left_tree('b');
    bt.insert_right_tree('e');

    let left = bt.get_left();
    println!("left val is {:#?}", left);
    let right = bt.get_right();
    println!("right val is {:#?}", right);
}
