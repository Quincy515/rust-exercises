/// 4. 🌟🌟🌟 裸指针可以和代表内存地址的整数互相转换


// 填空
fn main() {
    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address: usize = p1 as usize; 
    let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()
    let p2: *mut i32 = second_address as *mut i32; // p2 指向 values 数组中的第二个元素
    unsafe {
        // 将第二个元素加 1
        *p2 += 1;
    }
    
    assert_eq!(values[1], 3);

    println!("Success!")
}

