/// 梳排序 序时间复杂度 O(nlogn) 空间复杂度为 O(1)，属于不稳定的排序算法
/// 开始比较间距设定为数组长度，并在循环中以固定的比率递减，通常递减率为 1.3
/// 当间距为 1 时，梳排序就退化成了冒泡排序。
/// 梳排序通过尽量把逆序的数字往前移动并保证当前间隔内的数有序
fn comb_sort(nums: &mut [i32]) {
    if nums.len() <= 1 {
        return;
    }

    let mut i;
    let mut gap: usize = nums.len();

    // 大致排序，数据基本有序
    while gap > 0 {
        gap = (gap as f32 * 0.8) as usize;
        i = gap;
        while i < nums.len() {
            if nums[i - gap] > nums[i] {
                nums.swap(i - gap, i);
            }
            i += 1;
        }
    }

    // 细致调节部分无序数据，exchange 控制是否继续交换数据
    let mut exchange = true;
    while exchange {
        exchange = false;
        i = 0;

        while i < nums.len() - 1 {
            if nums[i] > nums[i + 1] {
                nums.swap(i, i + 1);
                exchange = true;
            }
            i += 1;
        }
    }
}

fn main() {
    let mut nums = [1, 2, 8, 3, 4, 9, 5, 6, 7];
    comb_sort(&mut nums);
    println!("sorted nums: {:?}", nums);
}
