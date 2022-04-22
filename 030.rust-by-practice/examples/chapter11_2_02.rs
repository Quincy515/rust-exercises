/// 2.🌟🌟 Vec 可以使用 extend 方法进行扩展 



// 填空
fn main() {
    let mut v1 = Vec::from([1, 2, 4]);
    v1.pop();
    v1.push(3);
    
    let mut v2 = Vec::new();
    v2.extend([1,2,3]);

    assert_eq!(v1, v2);

    println!("Success!")
}

