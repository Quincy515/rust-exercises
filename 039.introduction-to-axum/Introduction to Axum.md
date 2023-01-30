| Youtube 视频  | 代码仓库 |
|---|---|
| https://www.youtube.com/playlist?list=PLrmY5pVcnuE-_CP7XZ_44HN-mDrLQV4nS  |  https://github.com/brooks-builds/full-stack-todo-rust-course/blob/main/backend/rust/axum/README.md |

- [1. Hello World](#1-hello-world)
- [2. Auto Restarting the Server](#2-auto-restarting-the-server)
- [3. Local Documentation](#3-local-documentation)
- [4. Handing HTTP Methods](#4-handing-http-methods)
- [5. Extracting a String](#5-extracting-a-string)
- [6. Extracting JSON](#6-extracting-json)
- [7. Path Variables](#7-path-variables)
- [8. Query Parameters](#8-query-parameters)
- [9. Extracting Standard Headers](#9-extracting-standard-headers)
- [10. Extracting Custom Headers](#10-extracting-custom-headers)
- [11. CORS Middleware](#11-cors-middleware)
- [12. Shared Middleware Data](#12-shared-middleware-data)
- [13. Custom Middleware](#13-custom-middleware)
- [14. HTTP Status Codes](#14-http-status-codes)
- [15. 200 HTTP Codes](#15-200-http-codes)
- [16. Returning JSON](#16-returning-json)
- [17. Validating with  Serde](#17-validating-with--serde)
- [18. Creating a Database](#18-creating-a-database)
- [19. SeaORM](#19-seaorm)
- [20. Connecting SeaORM to the Database](#20-connecting-seaorm-to-the-database)
- [21. Generating SeaORM Models](#21-generating-seaorm-models)
- [22. Custom Extractors](#22-custom-extractors)
- [23. Passing Data to Handlers](#23-passing-data-to-handlers)
- [24. Inserting to the Database](#24-inserting-to-the-database)
- [25. Selecting One Item from the Database](#25-selecting-one-item-from-the-database)
- [26. Get all from the Database](#26-get-all-from-the-database)
- [27. Using SeaORM filters](#27-using-seaorm-filters)
- [28. Atomic Updates](#28-atomic-updates)
- [29. Partial Updates](#29-partial-updates)
- [30. Deleting Data](#30-deleting-data)
- [31. Soft Deleting Data](#31-soft-deleting-data)
- [32. Creating Account](#32-creating-account)
- [33. Logging In](#33-logging-in)
- [34. Guarding a Route](#34-guarding-a-route)
- [35. Logging Out](#35-logging-out)
- [36. Guarding in Middleware](#36-guarding-in-middleware)
- [37. Hashing Passwords](#37-hashing-passwords)
- [38. Using JWTs](#38-using-jwts)
- [39. Custom Errors](#39-custom-errors)
- [40. Deploying](#40-deploying)


## 1. Hello World
```shell
cargo new hello-world
cd hello-world
cargo add axum 
cargo add tokio -F macros -F rt-multi-thread 
```

https://docs.rs/axum/latest/axum/#example

```rust
use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

[代码变动](https://github.com/CusterFun/rust-exercises/commit/586278b00b67c1861161b739425b3c4ffc7bd8ce)
## 2. Auto Restarting the Server

全局安装 `cargo-watch`

```shell
cargo install cargo-watch
```

执行命令
```shell
❯ cargo watch -x run
[Running 'cargo run']
    Finished dev [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/hello-word`
```

```shell
cargo watch -q -c -w src/ -x run
```

- -q 不输出 cargo watch 本身的信息
- -c 每次运行前先清空屏幕
- -w 监视特定的文件和文件夹
- -x 执行 cargo 命令，默认执行 cargo check

## 3. Local Documentation
使文档本地到项目中

```shell
cargo doc
cargo doc --open
```

## 4. Handing HTTP Methods
新建 `lib.rs` 文件
```rust
use axum::{routing::get, Router};

pub async fn run() {
    let app = Router::new().route("/", get(|| async { "Hello World" }));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

修改 `main.rs`
```rust
use routing::run;

#[tokio::main]
async fn main() {
    run().await;
}
```

新建文件夹 `api` 和文件 `api/mod.rs`

```rust
use axum::{Router, routing::get};

pub fn create_routes() -> Router {
    Router::new().route("/", get(|| async {"Hello World"}))
}
```

修改 `lib.rs`

```rust
mod api;

use api::create_routes;

pub async fn run() {
    let app = create_routes();

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

新建 `api/hello_world.rs`

```rust
pub async fn hello_world() -> String {
    "Hello World from api".to_owned()
}
```

修改 `api/mod.rs`

```rust
pub mod hello_world;

use axum::{Router, routing::get};
use hello_world::hello_world;

pub fn create_routes() -> Router {
    Router::new().route("/", get(hello_world))
}
```

[代码变动](https://github.com/CusterFun/rust-exercises/commit/0afa6d192a2938970b66c1fcee3665a771235ae1)

## 5. Extracting a String
新建文件 `api/mirror_body_string.rs`

```rust
pub async fn mirror_body_string(body: String) -> String {
	body
}
```

修改 `api/mod.rs`

```rust
pub mod hello_world;
pub mod mirror_body_string;

use axum::{
    routing::{get, post},
    Router,
};
use hello_world::hello_world;
use mirror_body_string::mirror_body_string;

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
}
```

[代码变动](https://github.com/CusterFun/rust-exercises/commit/4cdd6196e620bc023a14da98e949f7c90d59f11a)

## 6. Extracting JSON

```shell
cargo add serde -F derive
```

新建文件 `api/mirror_body_json.rs`

```rust
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MirrorJson {
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MirrorJsonRes {
    message: String,
    message_from_server: String,
}

pub async fn mirror_body_json(Json(body): Json<MirrorJson>) -> Json<MirrorJsonRes> {
    Json(MirrorJsonRes {
        message: body.message,
        message_from_server: "Hello from server".to_owned(),
    })
}
```

修改 `api/mod.rs `

```rust
pub mod hello_world;
pub mod mirror_body_json;
pub mod mirror_body_string;

use axum::{
    routing::{get, post},
    Router,
};
use hello_world::hello_world;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
}
```

[代码变动](https://github.com/CusterFun/rust-exercises/commit/b83bf28f3a18b0ddec2f3af9a1b0c276993dc957)

## 7. Path Variables

新建 `api/path_variable.rs`

```rust
use axum::extract::Path;

pub async fn path_variable(Path(id): Path<u32>) -> String {
    id.to_string()
}

pub async fn hard_coded_path() -> String {
    "You get 15!".to_owned()
}
```

修改 `api/mod.rs `

```rust
pub mod hello_world;
pub mod mirror_body_json;
pub mod mirror_body_string;
pub mod path_variable;

use axum::{
    routing::{get, post},
    Router,
};
use hello_world::hello_world;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use path_variable::{hard_coded_path, path_variable};

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variable/:id", get(path_variable))
        .route("/path_variable/15", get(hard_coded_path))
}
```

[代码变动](https://github.com/CusterFun/rust-exercises/commit/cc760776921edfc0c72902ee83996a7e5c9db357)

## 8. Query Parameters

新增文件 `api/query_params.rs`

```rust
use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct QueryParams {
    message: Option<String>,
    id: Option<u32>,
}

pub async fn query_params(Query(query): Query<QueryParams>) -> Json<QueryParams> {
    Json(query)
}
```

变动 `api/mod.rs`

```rust
pub mod hello_world;
pub mod mirror_body_json;
pub mod mirror_body_string;
pub mod path_variable;
pub mod query_params;

use axum::{
    routing::{get, post},
    Router,
};
use hello_world::hello_world;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use path_variable::{hard_coded_path, path_variable};
use query_params::query_params;

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variable/:id", get(path_variable))
        .route("/path_variable/15", get(hard_coded_path))
        .route("/query_params", get(query_params))
}
```

[代码变动](https://github.com/CusterFun/rust-exercises/commit/c4f848b0e8e39ae02df17f39ce3b230e3958213d)

## 9. Extracting Standard Headers

添加 axum 的 features

```shell
cargo add axum -F headers
```

新建文件 `api/mirror_user_agent.rs`

```rust
use axum::{headers::UserAgent, TypedHeader};

pub async fn mirror_user_agent(TypedHeader(user_agent): TypedHeader<UserAgent>) -> String {
    user_agent.to_string()
}
```

变动 `api/mod.rs`

```rust
pub mod hello_world;
pub mod mirror_body_json;
pub mod mirror_body_string;
pub mod mirror_user_agent;
pub mod path_variable;
pub mod query_params;

use axum::{
    routing::{get, post},
    Router,
};
use hello_world::hello_world;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use mirror_user_agent::mirror_user_agent;
use path_variable::{hard_coded_path, path_variable};
use query_params::query_params;

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variable/:id", get(path_variable))
        .route("/path_variable/15", get(hard_coded_path))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
}
```

[代码变动](https://github.com/CusterFun/rust-exercises/commit/fc41878f6409d1632bb94095136e01b1a2d4b454)

## 10. Extracting Custom Headers

新建文件 `api/mirror_custom_header.rs`

```rust
use axum::http::{HeaderMap, HeaderValue};

pub async fn mirror_custom_header(headers: HeaderMap) -> String {
    let default_value = HeaderValue::from_static("null");
    let value = headers.get("x-token").unwrap_or(&default_value);
    value.to_str().unwrap().to_owned()
}
```

变动 `api/mod.rs`

```rust
pub mod hello_world;
pub mod mirror_body_json;
pub mod mirror_body_string;
pub mod mirror_custom_header;
pub mod mirror_user_agent;
pub mod path_variable;
pub mod query_params;

use axum::{
    routing::{get, post},
    Router,
};
use hello_world::hello_world;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use mirror_custom_header::mirror_custom_header;
use mirror_user_agent::mirror_user_agent;
use path_variable::{hard_coded_path, path_variable};
use query_params::query_params;

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variable/:id", get(path_variable))
        .route("/path_variable/15", get(hard_coded_path))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
}
```

[代码变动](https://github.com/CusterFun/rust-exercises/commit/14002ef2148fff64fded743e3ce6cb908f08e45c)

## 11. CORS Middleware

https://docs.rs/axum/latest/axum/middleware/index.html#commonly-used-middleware

https://docs.rs/tower-http/latest/tower_http/cors/index.html

```shell
cargo add tower-http -F cors
```

新建文件 `api/.rs`

```rust

```

变动 `api/mod.rs`

```rust

```

[代码变动](

## 12. Shared Middleware Data

新建文件 `api/.rs`

```rust

```

变动 `api/mod.rs`

```rust

```

[代码变动](

## 13. Custom Middleware

新建文件 `api/.rs`

```rust

```

变动 `api/mod.rs`

```rust

```

[代码变动](

## 14. HTTP Status Codes

新建文件 `api/.rs`

```rust

```

变动 `api/mod.rs`

```rust

```

[代码变动](

## 15. 200 HTTP Codes

新建文件 `api/.rs`

```rust

```

变动 `api/mod.rs`

```rust

```

[代码变动](

## 16. Returning JSON

新建文件 `api/.rs`

```rust

```

变动 `api/mod.rs`

```rust

```

[代码变动](

## 17. Validating with  Serde

新建文件 `api/.rs`

```rust

```

变动 `api/mod.rs`

```rust

```

[代码变动](

## 18. Creating a Database

新建文件 `api/.rs`

```rust

```

变动 `api/mod.rs`

```rust

```

[代码变动](

## 19. SeaORM

新建文件 `api/.rs`

```rust

```

变动 `api/mod.rs`

```rust

```

[代码变动](

## 20. Connecting SeaORM to the Database

新建文件 `api/.rs`

```rust

```

变动 `api/mod.rs`

```rust

```

[代码变动](

## 21. Generating SeaORM Models

新建文件 `api/.rs`

```rust

```

变动 `api/mod.rs`

```rust

```

[代码变动](

## 22. Custom Extractors

新建文件 `api/.rs`

```rust

```

变动 `api/mod.rs`

```rust

```

[代码变动](

## 23. Passing Data to Handlers

新建文件 `api/.rs`

```rust

```

变动 `api/mod.rs`

```rust

```

[代码变动](

## 24. Inserting to the Database

新建文件 `api/.rs`

```rust

```

变动 `api/mod.rs`

```rust

```

[代码变动](

## 25. Selecting One Item from the Database

新建文件 `api/.rs`

```rust

```

变动 `api/mod.rs`

```rust

```

[代码变动](

## 26. Get all from the Database

新建文件 `api/.rs`

```rust

```

变动 `api/mod.rs`

```rust

```

[代码变动](

## 27. Using SeaORM filters

新建文件 `api/.rs`

```rust

```

变动 `api/mod.rs`

```rust

```

[代码变动](

## 28. Atomic Updates

新建文件 `api/.rs`

```rust

```

变动 `api/mod.rs`

```rust

```

[代码变动](

## 29. Partial Updates

新建文件 `api/.rs`

```rust

```

变动 `api/mod.rs`

```rust

```

[代码变动](

## 30. Deleting Data 

新建文件 `api/.rs`

```rust

```

变动 `api/mod.rs`

```rust

```

[代码变动](

## 31. Soft Deleting Data

新建文件 `api/.rs`

```rust

```

变动 `api/mod.rs`

```rust

```

[代码变动](

## 32. Creating Account

新建文件 `api/.rs`

```rust

```

变动 `api/mod.rs`

```rust

```

[代码变动](

## 33. Logging In

新建文件 `api/.rs`

```rust

```

变动 `api/mod.rs`

```rust

```

[代码变动](

## 34. Guarding a Route

新建文件 `api/.rs`

```rust

```

变动 `api/mod.rs`

```rust

```

[代码变动](

## 35. Logging Out

新建文件 `api/.rs`

```rust

```

变动 `api/mod.rs`

```rust

```

[代码变动](

## 36. Guarding in Middleware

新建文件 `api/.rs`

```rust

```

变动 `api/mod.rs`

```rust

```

[代码变动](

## 37. Hashing Passwords

新建文件 `api/.rs`

```rust

```

变动 `api/mod.rs`

```rust

```

[代码变动](

## 38. Using JWTs

新建文件 `api/.rs`

```rust

```

变动 `api/mod.rs`

```rust

```

[代码变动](

## 39. Custom Errors

新建文件 `api/.rs`

```rust

```

变动 `api/mod.rs`

```rust

```

[代码变动](

## 40. Deploying

新建文件 `api/.rs`

```rust

```

变动 `api/mod.rs`

```rust

```

[代码变动](