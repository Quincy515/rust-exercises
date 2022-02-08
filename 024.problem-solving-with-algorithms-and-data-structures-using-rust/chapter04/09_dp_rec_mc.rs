fn dp_rec_mc(cashes: &[u32], amount: u32, min_cashes: &mut [u32]) -> u32 {
    // 动态收集从 1 到 amount 的最小找零币值数量
    // 然后从小到大凑出找零纸币数量
    for denm in 1..=amount {
        let mut min_cashe_num = denm;
        for c in cashes.iter().filter(|&c| *c <= denm).collect::<Vec<&u32>>() {
            let index = (denm - c) as usize;

            let cashe_num = 1 + min_cashes[index];
            if cashe_num < min_cashe_num {
                min_cashe_num = cashe_num;
            }
        }
        min_cashes[denm as usize] = min_cashe_num;
    }

    // 因为收集了各个值的最小找零纸币数，所以直接返回
    min_cashes[amount as usize]
}

fn main() {
    let amount = 81u32;
    let cashes: [u32; 5] = [1, 5, 10, 20, 50];
    let mut min_cashes: [u32; 82] = [0; 82];
    let cashe_num = dp_rec_mc(&cashes, amount, &mut min_cashes);
    println!("Refund for ${amount} need {cashe_num} cashes");
}
