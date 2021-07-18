fn main() {
    // 小胖的裤子
    let xiaopang_trousers = String::from("#########");
    // 新同事看裤子
    let xiaopang_trousers = new_colleague_look(xiaopang_trousers);
    // 小胖看裤子
    println!("这是我的裤子: {}", xiaopang_trousers);
}

fn new_colleague_look(trousers: String) -> String {
    println!("这裤子:{}真帅气!", trousers);
    trousers // 看过之后返回裤子
}
