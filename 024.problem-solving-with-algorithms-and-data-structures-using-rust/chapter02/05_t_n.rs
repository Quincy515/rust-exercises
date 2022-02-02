fn main() {
    // 分配操作数a,b,c,n的时间为常数4
    let a = 1;
    let b = 2;
    let c = 3;
    let n = 1000000;

    // 嵌套迭代，有三个语句执行n的平方次
    for i in 0..n {
        for j in 0..n {
            let x = i * i;
            let y = j * j;
            let z = i * j;
        }
    }

    // 有两个语句执行了n次
    for k in 0..n {
        let w = a * b + 45;
        let v = b * b;
    }

    // 常数1
    let d = 33;
}

// O(n^2)
