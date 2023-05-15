[axum-turtorial](./README.md)

- [setting up yew](#setting-up-yew)
  - [1. 首先在控制台执行](#1-首先在控制台执行)
  - [2. 然后安装 `cargo` 依赖](#2-然后安装-cargo-依赖)
  - [3. 在根目录下新建 `web/index.html`](#3-在根目录下新建-webindexhtml)
  - [4. 新增 `lib.rs` 和 `main.rs`](#4-新增-librs-和-mainrs)
  - [5. 运行服务，访问浏览器](#5-运行服务访问浏览器)
- [Bootstrap CSS](#bootstrap-css)

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
   
### 4. 新增 `lib.rs` 和 `main.rs`
在 `web/src/lib.rs` 中添加 HelloWorld

```rust
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}
```

在 `web/src/main.rs` 中添加初始化

```rust
use web::App;

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

打开浏览器访问 http://127.0.0.1:8080/

## Bootstrap CSS

https://getbootstrap.com/
https://getbootstrap.com/docs/5.3/getting-started/download/#cdn-via-jsdelivr

将 CDN 添加到 `index.html` 中

```html
<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0-alpha2/dist/css/bootstrap.min.css" rel="stylesheet"
        integrity="sha384-aFq/bzH65dt+w6FI2ooMVUpc+21e0SRygnTpmBvdBgSdnuTN7QbdgL+OapgHtvPp" crossorigin="anonymous">
</head>

<body>

    <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0-alpha2/dist/js/bootstrap.bundle.min.js"
        integrity="sha384-qKXV1j0HvMUeCBQ+QVp7JcfGl760yU08IQ+GpUo5hlbpg51QRiuqHAJz8+BrxE/N"
        crossorigin="anonymous"></script>
</body>

</html>
```

