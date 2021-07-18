fn main() {
    // 小胖的一箱裤子
    let mut xiaopang_box_trousers = vec![
        String::from("1.##########"),
        String::from("2.#########"),
        String::from("3.########"),
        String::from("4.###########"),
        String::from("5.############"),
    ];
    // 新同事看裤子 只看裤子的凭证
    new_colleague_look(&xiaopang_box_trousers[1..3]);
    println!("请女朋友修改裤子");
    girl_friend_change(&mut xiaopang_box_trousers[..3]);
    println!("请妈妈修改裤子");
    mother_change(&mut xiaopang_box_trousers[3..]);
    let xiaopang_other_trousers = String::from("6.#######");
    xiaopang_box_trousers.push(xiaopang_other_trousers);
    println!("现在箱子里有{}条裤子", xiaopang_box_trousers.len());
}

fn new_colleague_look(trousers_slice: &[String]) {
    for trouser in trousers_slice.iter() {
        println!("这裤子:{}真帅气!", trouser);
    }
}

fn girl_friend_change(trousers_slice: &mut [String]) {
    for trouser in trousers_slice.iter_mut() {
        trouser.truncate(9);
        println!("女朋友改好了裤子: {}", trouser);
    }
}

fn mother_change(trousers_slice: &mut [String]) {
    for trouser in trousers_slice.iter_mut() {
        trouser.truncate(9);
        println!("妈妈修改好了裤子: {}", trouser);
    }
}
