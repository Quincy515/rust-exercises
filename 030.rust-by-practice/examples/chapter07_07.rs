///continue and break
///7.🌟 使用 break 可以跳出循环


// 填空，不要修改其它代码
fn main() {
    let mut n = 0;
    for _i in 0..=100 {
       if n == 66 {
           break;
       }
       n += 1;
    }

    assert_eq!(n, 66);
}

