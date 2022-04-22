/// 练习
/// 1.🌟🌟 <T, const N: usize> 是结构体类型的一部分，和数组类型一样，
/// 这意味着长度不同会导致类型不同： Array<i32, 3> 和 Array<i32, 4> 是不同的类型


// 修复错误
struct Array<T, const N: usize> {
    data : [T; N]
}

fn main() {
    let _arrays = [
        Array{
            data: [1, 2, 3],
        },
        Array {
            data: [1, 2, 3],
        },
        Array {
            data: [1, 2, 3]
        }
    ];
    let _arrays = (
        Array{
            data: [1, 2, 3],
        },
        Array {
            data: [1.0, 2.0, 3.0],
        },
        Array {
            data: [1, 2]
        },
    );
}

