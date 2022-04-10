/// matches!
/// matches! 看起来像 match, 但是它可以做一些特别的事情

/// 4. 🌟🌟


fn main() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

    // 使用 `matches` 填空
    for ab in alphabets {
        assert!(match ab {
            _ => true,
        });
        assert!(matches!(ab, 'a'..='z' | 'A'..='Z' | '0'..='9'));
    }
} 

