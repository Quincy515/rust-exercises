use std::fmt::Debug;

/// 第一步 ， 将 待 排 序 元 素 划 分 到 不 同 的 桶 ， 先 遍 历求 出 maxV 和 minV，
/// 设桶 个 数为 k ， 则把区间 [ minV , maxV] 均匀 划 分成 k 个区间 ，
/// 每 个 区 间 就 是 一 个 桶 ， 将 序 列 中 的 元 素 分 配 到 各 自 的 桶 （ 求余法 ）。
/// 第二步 ， 对 每 个 桶 内 的 元 素 进 行 排 序 ， 排 序 算 法 可 用 任 意 排 序 算 法 。
/// 第三步 ， 将 各 个 桶 中 的 有 序 元 素 合 并 成 一 个 大 的 有 序 集 合 。

// hasher 是一个函数，计算时传入
// values 是数据容器，保存数据
struct Bucket<H, T> {
    hasher: H,
    values: Vec<T>,
}

impl<H, T> Bucket<H, T> {
    fn new(hasher: H, value: T) -> Bucket<H, T> {
        Bucket {
            hasher,
            values: vec![value],
        }
    }
}

/// 桶排序，Debug 特性是为了打印 T
fn bucket_sort<H, T, F>(nums: &mut [T], hasher: F)
where
    H: Ord,
    T: Ord + Clone + Debug,
    F: Fn(&T) -> H,
{
    let mut buckets: Vec<Bucket<H, T>> = Vec::new();

    for val in nums.iter() {
        let hasher = hasher(&val);

        // 对桶中数据二分搜索并排序
        match buckets.binary_search_by(|bct| bct.hasher.cmp(&hasher)) {
            Ok(idx) => buckets[idx].values.push(val.clone()),
            Err(idx) => buckets.insert(idx, Bucket::new(hasher, val.clone())),
        }
    }

    // 拆桶，将所有排序数据融合到一个 Vec
    let ret = buckets
        .into_iter()
        .flat_map(|mut bucket| {
            bucket.values.sort();
            bucket.values
        })
        .collect::<Vec<T>>();

    nums.clone_from_slice(&ret);
}

fn main() {
    let mut nums = [54, 32, 99, 18, 75, 31, 43, 56, 21, 22];
    bucket_sort(&mut nums, |t| t / 5);
    println!("sorted nums: {:?}", nums);
}
