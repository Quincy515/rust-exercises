fn main() {
    // 小胖的裤子
    let mut xiaopang_trousers = String::from("##########");
    // 新同事看裤子 只看裤子的凭证
    new_colleague_look(&xiaopang_trousers);
    // 小胖看裤子
    println!("这是我的裤子: {}", xiaopang_trousers);
    println!("裤子太长了，裤长为: {}", xiaopang_trousers.len());
    // girl_friend_change(&mut xiaopang_trousers);
    // mother_change(&mut xiaopang_trousers);
    // 可变引用同时只能存在一个，如下生命周期发生交集，会报错
    let mother_change_trousers = &mut xiaopang_trousers;
    let girl_friend_trousers = &mut xiaopang_trousers;
    mother_change_trousers.truncate(9);
    girl_friend_trousers.truncate(9);
    println!("现在裤子的长为: {}", xiaopang_trousers.len());
}

fn new_colleague_look(trousers: &String) {
    println!("这裤子:{}真帅气!", trousers);
}

fn girl_friend_change(trousers: &mut String) {
    // println!("这会出去找工作了，晚上回去改");
    // trousers.pop();
    trousers.truncate(9); // 幂等的方法
}

fn mother_change(trousers: &mut String) {
    // trousers.pop();
    trousers.truncate(9); // 幂等的方法
}
