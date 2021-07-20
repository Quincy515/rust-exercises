use std::sync::{mpsc::*, Arc, Mutex};
// mpsc 多个生产者单个消费者的缩写

fn create_skin(sender: Sender<String>) {
    loop {
        // 每200毫秒出一个饺子皮
        let skin = String::from("skin");
        sender.send(skin);
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}

fn create_stuffing(sender: Sender<String>) {
    loop {
        // 每200毫秒出一个饺子馅
        let stuff = String::from("stuffing");
        sender.send(stuff);
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}
fn make_dumplings(
    rec_skin: Receiver<String>,
    rec_stuffing: Receiver<String>,
    spoon_mtx: Arc<Mutex<String>>,
) {
    loop {
        {
            let _mtx = spoon_mtx.lock();
            let skin = rec_skin.recv().unwrap();
            println!("skin ok");
            std::thread::sleep(std::time::Duration::from_millis(50));
            let stuff = rec_stuffing.recv().unwrap();
            println!("stuffing ok");
            std::thread::sleep(std::time::Duration::from_millis(50));
            println!("dumplings ok [{}, {}]", skin, stuff);
        }
        std::thread::sleep(std::time::Duration::from_millis(400));
    }
}
fn main() {
    let (sender_skin, receiver_skin) = channel::<String>(); // 传输字符串类型的信息
    let (sender_stuff, receiver_stuff) = channel::<String>();
    let spoon_mtx = Arc::new(Mutex::new(String::from("spoon")));
    let spoon_mtx_a = Arc::clone(&spoon_mtx);
    let spoon_mtx_b = Arc::clone(&spoon_mtx);
    std::thread::spawn(|| create_skin(sender_skin));
    std::thread::spawn(|| create_stuffing(sender_stuff));
    std::thread::spawn(|| make_dumplings(receiver_skin, receiver_stuff, spoon_mtx_a));
    std::thread::spawn(move || {
        {
            let _mtx = spoon_mtx_b.lock();
            println!("play");
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    });
    std::thread::sleep(std::time::Duration::from_millis(3000));
}
