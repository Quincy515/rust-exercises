/// 复杂度为 nlog2(n) 最坏的情况下，复杂度为 O(n^2)。
fn quick_sort1(nums: &mut [i32], low: usize, high: usize) {
    if low < high {
        let split = partition(nums, low, high);
        if split > 1 {
            // 防止越界和语法错误（split <= 1 的情形）
            quick_sort1(nums, low, split - 1);
        }
        quick_sort1(nums, split + 1, high);
    }
}

fn partition(nums: &mut [i32], low: usize, high: usize) -> usize {
    let mut lm = low; // 左标记
    let mut rm = high; // 右标记
    loop {
        // 左标记不断右移
        while lm <= rm && nums[lm] <= nums[low] {
            lm += 1;
        }
        // 右标记不断左移
        while lm <= rm && nums[rm] >= nums[low] {
            rm -= 1;
        }

        // 左标记越过右标记时退出并交换左右标记数据
        if lm > rm {
            break;
        } else {
            nums.swap(lm, rm);
        }
    }
    nums.swap(low, rm);
    rm
}

fn main() {
    let mut nums = [54, 32, 99, 18, 75, 31, 43, 56, 21, 22];
    let len = nums.len();
    quick_sort1(&mut nums, 0, (len - 1) as usize);
    println!("sorted nums: {:?}", nums);
}
