// 等值比较 Eq(等价关系) 与 PartialEq(局部等价关系)
// 两个相同的浮点类型和 两个非数字值 NaN 是互不相等的，NaN!=NaN
enum BookFormat {
    Paperback,
    Hardback,
    Ebook,
}
struct Book {
    isbn: i32,
    format: BookFormat,
}

// 为 Book 自定义实现 PartialEq 用于判断两个实例是否相等的 eq 方法
// 只要字段 isbn 值相等，即使字段 format 值不同，两本书也视为同一本书。
impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.isbn == other.isbn
    }
}

fn main() {
    let b1 = Book {
        isbn: 3,
        format: BookFormat::Paperback,
    };
    let b2 = Book {
        isbn: 3,
        format: BookFormat::Ebook,
    };
    let b3 = Book {
        isbn: 5,
        format: BookFormat::Paperback,
    };

    assert!(b1 == b2);
    assert!(b1 != b3);
}
