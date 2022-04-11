use futures::executor::block_on;

async fn hello_world() {
    println!("hello, world!");
}
fn main() {
    let future = hello_world(); // 什么都没打印出来
    block_on(future); // `future` 运行，并打印出 "hello, world"
}
