fn main() {
    let arr1: [i32; 5] = [1, 2, 3, 4, 5];
    let arr2 = [1, 2, 3, 4, 5];
    let arr3: [i32; 5] = [1; 5];
    let arr4 = [1; 5];

    println!("{:?}", arr1); // [1, 2, 3, 4, 5]
    println!("{:?}", arr2); // [1, 2, 3, 4, 5]
    println!("{:?}", arr3); // [1, 1, 1, 1, 1]
    println!("{:?}", arr4); // [1, 1, 1, 1, 1]
    println!("arr1[0]: {}, arr3[2]: {}", arr1[0], arr3[2]);
}
