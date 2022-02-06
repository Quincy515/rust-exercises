/// 选择排序 O(n^2)
/// 每次遍历时，选择未排序的最大的项，然后放置在适当位置。
/// 由于选择排序每轮只进行一次数据交换，所以比冒泡排序更快
fn selection_sort(nums: &mut Vec<i32>) {
    let mut left = nums.len() - 1; // 待排序数据下标
    while left > 0 {
        let mut pos_max = 0;
        for i in 1..=left {
            if nums[i] > nums[pos_max] {
                pos_max = i; // 选择当前轮次最大值下标
            }
        }
        // 数据交换，完成一个数据的排序，待排序数据量减 1
        nums.swap(left, pos_max);
        left -= 1;
    }
}

fn main() {
    let mut nums = vec![54, 32, 99, 18, 75, 31, 43, 56, 21, 22];
    selection_sort(&mut nums);
    println!("sorted nums: {:?}", nums);
}
