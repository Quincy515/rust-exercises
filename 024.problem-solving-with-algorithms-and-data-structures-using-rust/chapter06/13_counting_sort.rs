/// 第一步 ， 初 始 化 长 度 为 maxV − minV + 1 的 计 数 器 集 合 ， 值全为 0，
/// 其中 maxV 为 待 排 序 集 合 的 最 大 值 ，minV 为最小值 。
/// 第二步 ， 扫 描 待 排 序 集 合 ， 以 当 前值 减 minV 作下标 ， 并 对 计 数 器 中
/// 此 下 标 的 计 数 加 1。
/// 第三步 ， 扫 描 一 遍 计 数 器 集 合 ， 按 顺 序 把 值 写 回 原 集 合 ， 完成排序 。
fn counting_sort(nums: &mut [usize]) {
    if nums.len() <= 0 {
        return;
    }

    // 桶数量为 nums 中最大值加1，保证数据都有桶放
    let max_bkt_num = nums.iter().max().unwrap() + 1;
    let mut counter = vec![0; max_bkt_num];
    for &v in nums.iter() {
        counter[v] += 1; // 将数据标记到桶
    }

    // 数据写回原 nums 切片
    let mut j = 0;
    for i in 0..max_bkt_num {
        while counter[i] > 0 {
            nums[j] = i;
            counter[i] -= 1;
            j += 1;
        }
    }
}

fn main() {
    let mut nums = [54, 32, 99, 18, 75, 31, 43, 56, 21, 22];
    counting_sort(&mut nums);
    println!("sorted nums: {:?}", nums);
}
