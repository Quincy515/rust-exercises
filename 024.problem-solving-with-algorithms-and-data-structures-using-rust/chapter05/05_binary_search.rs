fn binary_search1(nums: &[i32], num: i32) -> bool {
    let mut low = 0;
    let mut high = nums.len() - 1;
    let mut found = false;

    // 注意是 <= 不是 <
    while low <= high && !found {
        let mid: usize = (low + high) >> 1;

        // 若 low + high 可能溢出，可转换为减法
        // let mid: usize = low + ((heigh - low) >> 1);

        if num == nums[mid] {
            found = true;
        } else if num < nums[mid] {
            high = mid - 1; // num 小于中间值，省去后半部数据
        } else {
            low = mid + 1; // num 大于等于中间值，省去前半部数据
        }
    }
    found
}

fn main() {
    let nums = [1, 3, 8, 10, 15, 32, 44, 48, 50, 55, 60, 62, 64];
    let target = 3;
    let found = binary_search1(&nums, target);
    println!("{target} is in nums: {found}");

    let target = 63;
    let found = binary_search1(&nums, target);
    println!("{target} is in nums: {found}");
}
