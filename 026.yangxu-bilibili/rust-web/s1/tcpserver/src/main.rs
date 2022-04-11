use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    // 创建 listener 绑定到本地端口 3000
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000...");

    // 通过 incoming 方法循环接收连接（字节流 TcpStream）
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established!");
        let mut buffer = [0;1024];

        // 把传入的 stream 读入到 buffer 里面
        stream.read(&mut buffer).unwrap();
        // 把得到的内容原封不动的返回
        stream.write(&mut buffer).unwrap();
    }
}
