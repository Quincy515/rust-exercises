[axum-turtorial](./README.md)

- [setting up yew](#setting-up-yew)
  - [1. 首先在控制台执行](#1-首先在控制台执行)
  - [2. 然后安装 `cargo` 依赖](#2-然后安装-cargo-依赖)
  - [3. 在根目录下新建 `web/index.html`](#3-在根目录下新建-webindexhtml)
  - [4. 在 `web/src/main.rs` 中添加初始化](#4-在-websrcmainrs-中添加初始化)
  - [5. 运行服务，访问浏览器](#5-运行服务访问浏览器)

## setting up yew

https://yew.rs/docs/next/getting-started/introduction

### 1. 首先在控制台执行

```shell
rustup target add wasm32-unknown-unknown
cargo install trunk
cargo install wasm-bindgen-cli
```

### 2. 然后安装 `cargo` 依赖

```shell
cargo add yew --features csr 
cargo add yewdux 
cargo add yew-router
cargo add stylist --features yew
cargo add serde -F derive
cargo add serde-json
cargo add thiserror
cargo add gloo 
cargo add gloo-net
cargo add js-sys
cargo add wasm-bindgen
cargo add wasm-bindgen-futures
cargo add web-sys --features HtmlInputElement 
```

然后把 `Cargo.toml` 文件修改为

```toml
[package]
name = "web"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
gloo = "0.8"
gloo-net = "0.2"
js-sys = "0.3"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
stylist = { version = "0.12", features = ["yew"] }
thiserror = "1"
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
yew = { version = "0.20", features = ["csr"] }
yew-router = "0.17"
yewdux = "0.9"

[dependencies.web-sys]
features = ["HtmlInputElement", "HtmlSelectElement"]
version = "0.3"
```

### 3. 在根目录下新建 `web/index.html`

```html
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8" />
        <title>myapp</title>
    </head>
</html>
```
   
### 4. 在 `web/src/main.rs` 中添加初始化

```rust
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```

此时目录结构为

```shell
├── Cargo.lock
├── Cargo.toml
├── README.md
├── index.html
├── src
│   └── main.rs
```

### 5. 运行服务，访问浏览器

```shell
trunk serve
```

打开浏览器访问 http://localhost:8080