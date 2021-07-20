use actix::*;
use std::sync::{mpsc::*, Arc, Mutex};
// mpsc 多个生产者单个消费者的缩写

// 1. 定义演员
struct Jiejie {
    left_hand: Vec<String>,
    right_hand: Vec<String>,
}
impl Jiejie {
    fn new() -> Self {
        Self {
            left_hand: vec![],
            right_hand: vec![],
        }
    }
    fn make_dumpl(&mut self) {
        while !self.left_hand.is_empty() && !self.right_hand.is_empty() {
            println!("[{:?}, {:?}]", self.left_hand.pop(), self.right_hand.pop());
        }
    }
}
impl Actor for Jiejie {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        println!("I am started");
    }
}
// 2. 定义消息
struct DumplMsg {
    content: String,
}
impl Message for DumplMsg {
    type Result = String;
}
// 3. 定义演员处理的消息类型和处理消息的方法
impl Handler<DumplMsg> for Jiejie {
    type Result = String;
    fn handle(&mut self, msg: DumplMsg, ctx: &mut Context<Self>) -> Self::Result {
        if msg.content.starts_with("skin") {
            self.left_hand.push(msg.content);
        } else if msg.content.starts_with("stuff") {
            self.right_hand.push(msg.content);
        }
        self.make_dumpl();
        String::from("ok")
    }
}
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
    // 4. 创建系统也就是舞台
    let system = actix::System::new("make dumpl");
    // 5. 实例化演员
    let addr = Jiejie::new().start();
    std::thread::spawn(move || loop {
        addr.do_send(DumplMsg {
            content: String::from("skin"),
        });
        addr.do_send(DumplMsg {
            content: String::from("stuff"),
        });
        std::thread::sleep(std::time::Duration::from_millis(2000));
    });
    // 6. 启动系统舞台，开始表演
    system.run();
}
