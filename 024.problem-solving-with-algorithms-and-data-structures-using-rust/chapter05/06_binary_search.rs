/// 二分查找时，找到或没找到是最终的结果，是一个基本情况。
/// 而二分法不断减小问题的尺度，不断向基本情况靠近，
/// 且二分法是在不断重复自身步骤
/// 用递归实现二分法
fn binary_search2(nums: &[i32], num: i32) -> bool {
    // 基本情况1：项不存在
    if 0 == nums.len() {
        return false;
    }

    let mid: usize = nums.len() >> 1;

    // 基本情况2：项存在
    if num == nums[mid] {
        return true;
    } else if num < nums[mid] {
        // 减小问题规模
        return binary_search2(&nums[..mid], num);
    } else {
        return binary_search2(&nums[mid + 1..], num);
    }
}

fn main() {
    let nums = [1, 3, 8, 10, 15, 32, 44, 48, 50, 55, 60, 62, 64];
    let target = 3;
    let found = binary_search2(&nums, target);
    println!("{target} is in nums: {found}");

    let target = 63;
    let found = binary_search2(&nums, target);
    println!("{target} is in nums: {found}");
}
