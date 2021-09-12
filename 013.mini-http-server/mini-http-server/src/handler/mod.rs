use tokio::net::{TcpStream, TcpListener};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::fs::File;

const CRLF: &str = "\r\n";

pub async fn handle_request(mut stream: TcpStream) {
    let mut buf = [0; 4096];
    stream.read(&mut buf).await.unwrap();

    let write = |(contents, status)| write(stream, contents, status);

    // 路由处理
    if matched(&buf, "/index") {
        write(handle_index().await).await
    } else {
        write(handle_404().await).await
    }
}

// 路由匹配
fn matched(buf: &[u8; 4096], route: &str) -> bool {
    let s = format!("GET {} HTTP/1.1{}", route, CRLF);
    buf.starts_with(s.as_bytes())
}

// 处理 index，返回首页
async fn handle_index() -> (String, String) {
    (read_html_file("index.html").await, status(200, "OK"))
}

async fn handle_404() -> (String, String) {
    (String::from("404"), status(200, "OK"))
}

// 读 html 文件
async fn read_html_file(file_name: &str) -> String {
    let mut file = File::open(file_name).await.unwrap();
    let mut html_contents = String::new();
    file.read_to_string(&mut html_contents).await.unwrap();
    html_contents
}

fn status(code: i32, text: &str) -> String {
    format!("HTTP/1.1 {} {}{}", code, text, CRLF)
}

// 将响应写出道流
async fn write(mut stream: TcpStream, contents: String, status: String) {
    let content_type = format!("Content-Type: text/html;charset=utf-8{}", CRLF);
    let server = format!("Server: Rust{}", CRLF);
    let content_length = format!("Content-Length {}{}", contents.as_bytes().len(), CRLF);
    let response = format!("{0}{1}{2}{3}{4}{5}", status, server, content_type, content_length, CRLF, contents);
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}