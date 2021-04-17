> 原文：[Rust Programming Language Tutorial – How to Build a To-Do List App](https://www.freecodecamp.org/news/how-to-build-a-to-do-app-with-rust/)
>
> 译文：[Rust 语言入门教程：从实战 To-Do App 开始](https://mp.weixin.qq.com/s/MfDhZcZdB_mxND-yHwuKxw)

run and saved as JSON.

```rust
cargo run -- add "learn rust" json
cargo run -- add "to do list" json
cat db.json
{
  "to do list": true,
  "learn rust": true
}
cargo run -- complete "to do list" json
cat db.json
{
  "to do list": false,
  "learn rust": true
}
```

run and saved as txt.

```rust
cargo run -- add "learn rust" txt
cargo run -- add "to do list" txt
cat db.txt

cargo run -- complete "to do list" txt
cat db.txt

```

