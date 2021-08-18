use tokio::runtime::Runtime;

async fn roast_chicken() -> String {
    std::thread::sleep(std::time::Duration::from_millis(2000));
    String::from("roast chicken 2kg")
}
async fn deli_home(chicken: String) {
    println!("chicken is here");
}

fn main() {
    let rt = Runtime::new().unwrap();
    rt.spawn(async {
        let chicken = roast_chicken().await;
        deli_home(chicken).await;
    });
    println!("buy wine");
}
