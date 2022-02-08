/// 指数查找是另一种二分查找的变体，
/// 它划分中值的方法不是使用平均或插值而是用指数函数来估计，
/// 这样可以快速找到上界，加快查找，该算法适合已排序且无边界的数据
fn exponential_search(nums: &[i32], target: i32) -> bool {
    let size = nums.len();
    if size == 0 {
        return false;
    }

    // 逐步找到上界
    let mut high = 1usize;
    while high < size && nums[high] < target {
        high <<= 1;
    }

    // 上界的一半一定可以作为下界
    let low = high >> 1;

    // 使用前面实现的二分查找
    binary_search(&nums[low..size.min(high + 1)], target)
}

fn main() {
    let nums = [1, 9, 10, 15, 16, 17, 19, 23, 27, 28, 29, 30, 32, 35];
    let target = 27;
    let found = exponential_search(&nums, target);
    println!("{target} is in nums: {found}");
}

fn binary_search(nums: &[i32], num: i32) -> bool {
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
