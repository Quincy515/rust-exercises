/// 为鸡尾酒排序
/// 从左到右是升序排序，从右到左采取降序排序
/// 冒泡排序的优化，复杂度还是 O(n^2)，若序列已经排序，则接近 O(n)
fn cocktail_sort(nums: &mut [i32]) {
    if nums.len() <= 1 {
        return;
    }

    // bubble 控制是否继续冒泡
    let mut bubble = true;
    let len = nums.len();
    for i in 0..(len >> 1) {
        if bubble {
            bubble = false;

            // 从左到右冒泡
            for j in i..(len - i - 1) {
                if nums[j] > nums[j + 1] {
                    nums.swap(j, j + 1);
                    bubble = true
                }
            }

            // 从右到左冒泡
            for j in (i + 1..=(len - i - 1)).rev() {
                if nums[j] < nums[j - 1] {
                    nums.swap(j - 1, j);
                    bubble = true
                }
            }
        } else {
            break;
        }
    }
}

fn main() {
    let mut nums = [1, 3, 2, 8, 3, 6, 4, 9, 5, 10, 6, 7];
    cocktail_sort(&mut nums);
    println!("sorted nums: {:?}", nums);
}
