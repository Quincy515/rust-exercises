use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;


fn main() {
   // 使用 TcpStream 上的 connect 方法建立链接，并返回一个 stream
   let mut stream = TcpStream::connect("localhost:3000").unwrap();
   // 把 hello 原始字节传输到服务器
   stream.write("Hello".as_bytes()).unwrap();

   // 从服务器接收消息
   let mut buf = [0; 5];
   stream.read(&mut buf).unwrap();

   // 打印服务器的响应
   println!("Response from server:{:?}", str::from_utf8(&buf).unwrap())
}
