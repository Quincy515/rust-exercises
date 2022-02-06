/// 时间复杂度 O(n^2)
/// 假设开始的子序列只有一项，位置为 0。
/// 在下次遍历时，对于项 1 至 n-1，将其与第一项比较，如果小于该项，则将其插入到该项前。
/// 如果大于该项，则增长子序列，使长度加一。
/// 接着重复比较过程，在剩余的未排序项里取数据来比较。
/// 结果是要么插入子序列某个位置，要么增长子序列，最终得到排好序的集合。
fn insertion_sort(nums: &mut [i32]) {
    for i in 1..nums.len() {
        let mut pos = i;
        let curr = nums[i];
        while pos > 0 && curr < nums[pos - 1] {
            nums[pos] = nums[pos - 1]; // 向后移动数据
            pos -= 1;
        }
        nums[pos] = curr; // 插入数据
    }
}

fn main() {
    let mut nums = [54, 32, 99, 18, 75, 31, 43, 56, 21, 22];
    insertion_sort(&mut nums);
    println!("sorted nums: {:?}", nums);
}
