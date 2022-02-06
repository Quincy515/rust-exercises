/// 插入排序总是需要和前面已排序的元素逐个比较来找到具体位置
/// 用二分查找法，可以快速找到元素在已排序子序列中的位置，
/// 所以可以利用二分法来加快插入排序的速度。
/// 具体的改动就是在插入时，用二分法找到数据该插入的位置，然后直接移动数据到相应位置。
fn bin_insertion_sort(nums: &mut [i32]) {
    let mut temp;
    let mut left;
    let mut mid;
    let mut right;

    for i in 1..nums.len() {
        left = 0; // 已排序数据左右边界
        right = i - 1;
        temp = nums[i]; // 待排序数据

        // 二分法找到 temp 的位置
        while left <= right {
            mid = (left + right) >> 1;
            if temp < nums[mid] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        // 将数据后移，留出空位
        for j in (left..=i - 1).rev() {
            nums.swap(j, j + 1);
        }

        // 将 temp 插入空位
        if left != i {
            nums[left] = temp;
        }
    }
}

fn main() {
    let mut nums = [1, 3, 2, 8, 6, 4, 9, 7, 5, 10];
    bin_insertion_sort(&mut nums);
    println!("sorted nums: {:?}", nums);
}
