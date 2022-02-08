/// 希尔排序中，增量是关键，也是其特征。
/// 可以使用不同的增量，但增量为几，那么子集合就有几个。
/// 通过不断调整 gap 值，实现排序。
/// 换句话说，随着 gap 值向 1 靠拢，整个集合都比上一次更有序，这使得总的排序非常高效
fn shell_sort(nums: &mut [i32]) {
    // 插入排序函数（内部），数据相隔距离为 gap
    fn ist_sort(nums: &mut [i32], start: usize, gap: usize) {
        let mut i = start + gap;
        while i < nums.len() {
            let mut pos = i;
            let curr = nums[pos];
            while pos >= gap && curr < nums[pos - gap] {
                nums[pos] = nums[pos - gap];
                pos -= gap;
            }

            nums[pos] = curr;
            i += gap;
        }
    }

    // 每次让 gap 减少一半直到 1
    let mut gap = nums.len() / 2;
    while gap > 0 {
        for start in 0..gap {
            ist_sort(nums, start, gap);
        }
        gap /= 2;
    }
}

fn main() {
    let mut nums = [54, 32, 99, 18, 75, 31, 43, 56, 21, 22];
    shell_sort(&mut nums);
    println!("sorted nums: {:?}", nums);
}
