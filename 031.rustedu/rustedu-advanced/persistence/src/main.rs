// 1个数据结构 他有自己的数据
// 还有自己的一些操作。这些操作就维持了数据一些结构保证它的一些特性
// stack push pop
// queue push pop
// 持久化的数据结构就是 可以有多个数据结构（历史版本）它们可以共享一些 data
// 保持数据结构的历史性
// List, treap, segment tree, ...
// n1 -> n2 -> n3 -> n4

// pretend
// n4: v1
// n3 -> n4: v2
// n2 -> n3 -> n4: v3
// n1 -> n2 -> n3 -> n4: v4
// list: n1 -> n2 -> n3 -> n4: v4（指针指向 n4）
// 想知道 v1 版本的 list
// 得到一个新的返回值 list_v1: n4
// v2:版本的 list_v2: n3 -> n4
// v5: 版本 pop 一个出去 n2 -> n3 -> n4
// v6: n6 -> n2 -> n3 -> n4）
// v6 版本怎么得到 v4 版本的 list
// n1 -> n2 -> n3 -> n4
//        ^
//        |
//        n6
// 我们怎么实现一个持久化的链表

use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn prepend(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

#[test]
fn iter() {
    let list = List::new().prepend(1).prepend(2).prepend(3);

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
}

// v6 版本的 list
// n6 -> n2 -> n3 -> n4
// Rc 没有 DerefMut
// Box 有 DerefMut
// take 它用的 mem::replace
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(boxed_node) = cur_link {
            // rc_node: mut Rc<Node<T>>
            // replace(rc_node, None)
            if let Ok(mut boxed_node) = Rc::try_unwrap(boxed_node) {
                cur_link = boxed_node.next.take();
            } else {
                break;
            }
        }
    }
}

// impl<T> Drop for List<T> {
//     fn drop(&mut self) {
//         let mut head = self.head.take();
//         while let Some(node) = head {
//             if let Ok(mut node) = Rc::try_unwrap(node) {
//                 head = node.next.take();
//             } else {
//                 break;
//             }
//         }
//     }
// }
