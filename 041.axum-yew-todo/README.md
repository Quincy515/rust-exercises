- [1.Introduce the project](#1introduce-the-project)
- [2.Hello World](#2hello-world)

## 1.Introduce the project

`axum` 基础知识 [导航](../039.introduction-to-axum/README.md)

```shell
cargo install cargo-watch
cargo install sea-orm-cli
cargo install cross
```

新建项目

```shell
mkdir axum-yew-todo
cd axum-yew-todo
cargo new server
cd server
touch .env
code .env
```

编辑 `.env` 文件

```text
exprot API_PORT=3000
export API_URI=http://localhost
export JWT_SECRET=jwt_secret
export DATABASE_URL=postgres://postgres:postgres@localhost/axum_yew_todo
```

```shell
cargo add axum -F headers -F macros
cargo add bcrypt
cargo add chrono -F serde
cargo add dotenvy
cargo add dotenvy_macro
cargo add jsonwebtoken
cargo add sea-orm -F sqlx-postgres -F runtime-tokio-rustls
cargo add serde -F derive
cargo add serde_with
cargo add tokio -F macros -F rt-multi-thread
cargo add tower-http -F cors
cargo add validator -F derive
```

本地文档

```shell
cargo doc
cargo doc --open
```

运行

```shell
cargo watch -q -c -w src -x run
```

或者

```shell
npx nodemon --watch src -e rs --exec cargo run
```

跨平台编译，需要先启动 `docker` 然后运行

```shell
cross build --release --target x86_64-unknown-linux-musl
```

## 2.Hello World

```shell
├─ src 
│  ├─ api 
│  │  ├─ mod.rs
│  │  └─ hello.rs
│  ├─ main.rs
│  ├─ router.rs
│  ├─ lib.rs
│  └─ .env
└─ Cargo.toml
```
代码详情
<details><summary>.env</summary>

```text
exprot API_PORT=3000
export API_URI=http://localhost
export JWT_SECRET=jwt_secret
export DATABASE_URL=postgres://postgres:postgres@localhost/axum_yew_todo
```
</details>
<details><summary>mod.rs</summary>

```rust
pub mod hello;
```
</details>

<details><summary>hello.rs</summary>

```rust
pub async fn hello() -> String {
    "Hello World!".to_string()
}
```
</details>

<details><summary>main.rs</summary>

```rust
use dotenvy::dotenv;
use server::run;

#[tokio::main]
async fn main() {
    dotenv().ok();
    run().await;
}
```
</details>

<details><summary>router.rs</summary>

```rust
use axum::{routing::get, Router};

use crate::api::hello::hello;

pub async fn create_router() -> Router {
    Router::new().route("/", get(hello))
}
```
</details>

<details><summary>lib.rs</summary>

```rust
use dotenvy_macro::dotenv;

mod api;
mod router;

pub async fn run() {
    let app = router::create_router().await;
    let addr = format!("{}:{}", dotenv!("API_URI"), dotenv!("API_PORT"));
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```
</details>

<details><summary>Cargo.toml</summary>

```toml
[package]
edition = "2021"
name = "server"
version = "0.1.0"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
axum = {version = "0.6.4", features = ["headers", "macros"]}
bcrypt = "0.14.0"
chrono = {version = "0.4.23", features = ["serde"]}
dotenvy = "0.15.6"
dotenvy_macro = "0.15.1"
jsonwebtoken = "8.2.0"
sea-orm = {version = "0.10.7", features = ["sqlx-postgres", "runtime-tokio-rustls"]}
serde = {version = "1.0.152", features = ["derive"]}
serde_with = "2.2.0"
tokio = {version = "1.25.0", features = ["macros", "rt-multi-thread"]}
tower-http = {version = "0.3.5", features = ["cors"]}
validator = {version = "0.16.0", features = ["derive"]}
```
</details>