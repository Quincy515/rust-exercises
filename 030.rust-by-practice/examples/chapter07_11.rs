/// 11. 🌟🌟🌟 当有多层循环时，
/// 你可以使用 continue 或 break 来控制外层的循环。
/// 要实现这一点，外部的循环必须拥有一个标签 'label, 
/// 然后在 break 或 continue 时指定该标签


// 填空
fn main() {
    let mut count = 0;
    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                // 这只会跳出 inner1 循环
                break 'inner1; // 这里使用 `break` 也是一样的
            }
            count += 2;
        }

        count += 5;

        'inner2: loop {
            if count >= 30 {
                break 'outer;
            }

            continue 'outer;
        }
    }

    assert!(count == 30)
}

