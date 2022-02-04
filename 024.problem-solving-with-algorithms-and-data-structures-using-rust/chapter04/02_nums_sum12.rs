/// nums_sum1 使用 nums[0] 加剩下的项来求和
fn nums_sum1(nums: &[i32]) -> i32 {
    if 1 == nums.len() {
        nums[0]
    } else {
        let first = nums[0];
        first + nums_sum1(&nums[1..])
    }
}

/// nums_sum2 使用最后一项和前面所有项来求和
fn nums_sum2(nums: &[i32]) -> i32 {
    if 1 == nums.len() {
        nums[0]
    } else {
        let last = nums[nums.len() - 1];
        nums_sum2(&nums[..nums.len() - 1]) + last
    }
}

fn main() {
    let nums = [2, 1, 7, 4, 5];
    let sum1 = nums_sum1(&nums);
    let sum2 = nums_sum2(&nums);
    println!("sum1 is {sum1}, sum2 is {sum2}");
}

/// 递归三定律
/// 递归算法必须具有基本情况
/// 递归算法必须向基本情况靠近
/// 递归算法必须以递归方式调用自身