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
- [17. Validating with Serde](#17-validating-with-serde)
- [18. Creating a Database](#18-creating-a-database)
- [19. SeaORM](#19-seaorm)
- [20. Connecting SeaORM to the Database](#20-connecting-seaorm-to-the-database)
- [21. Generating SeaORM Models](#21-generating-seaorm-models)
- [22. Custom Extractors](#22-custom-extractors)
  - [延伸 验证失败，返回 200 和自定义 json 数据](#延伸-验证失败返回-200-和自定义-json-数据)
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

<details><summary>变动 `api/mod.rs`</summary>

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

</details>

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

<details><summary>变动 `api/mod.rs`</summary>

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
</details>

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

<details><summary>变动 `api/mod.rs`</summary>

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
</details>

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

<details><summary>变动 `api/mod.rs`</summary>

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
</details>

[代码变动](https://github.com/CusterFun/rust-exercises/commit/c4f848b0e8e39ae02df17f39ce3b230e3958213d)

## 9. Extracting Standard Headers

添加 `axum` 的 `features`

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

<details><summary>变动 `api/mod.rs`</summary>

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
</details>

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

<details><summary>变动 `api/mod.rs`</summary>

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
</details>

[代码变动](https://github.com/CusterFun/rust-exercises/commit/14002ef2148fff64fded743e3ce6cb908f08e45c)

## 11. CORS Middleware

https://docs.rs/axum/latest/axum/middleware/index.html#commonly-used-middleware

https://docs.rs/tower-http/latest/tower_http/cors/index.html

```shell
cargo add tower-http -F cors
```

<details><summary>变动 `api/mod.rs`</summary>

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
use tower_http::cors::{Any, CorsLayer};

pub fn create_routes() -> Router {
    let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any);
    Router::new()
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variable/:id", get(path_variable))
        .route("/path_variable/15", get(hard_coded_path))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .layer(cors)
}
```
</details>

## 12. Shared Middleware Data

新建文件 `api/middleware_message.rs`

```rust
use axum::Extension;

use super::SharedData;

pub async fn middleware_message(Extension(shared_data): Extension<SharedData>) -> String {
    shared_data.message
}
```

<details><summary>变动 `api/mod.rs`</summary>

```rust
pub mod hello_world;
pub mod middleware_message;
pub mod mirror_body_json;
pub mod mirror_body_string;
pub mod mirror_custom_header;
pub mod mirror_user_agent;
pub mod path_variable;
pub mod query_params;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use hello_world::hello_world;
use middleware_message::middleware_message;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use mirror_custom_header::mirror_custom_header;
use mirror_user_agent::mirror_user_agent;
use path_variable::{hard_coded_path, path_variable};
use query_params::query_params;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

pub fn create_routes() -> Router {
    let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any);

    let shared_data = SharedData {
        message: "Hello from shared data".to_owned(),
    };

    Router::new()
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variable/:id", get(path_variable))
        .route("/path_variable/15", get(hard_coded_path))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .route("/middleware_message", get(middleware_message))
        .layer(Extension(shared_data))
        .layer(cors)
}
```
</details>

[代码变动](https://github.com/CusterFun/rust-exercises/commit/2bad6b281178528354e6f9a97805dd4ff68c8916)

## 13. Custom Middleware

> 目标： 
> 通过添加自定义中间件访问自定义的header

新建文件 `api/custom_middleware.rs`

```rust
use axum::Extension;

pub struct HeaderMessage(String);

pub async fn custom_middleware(Extension(message): Extension<HeaderMessage>) -> String {
    message.0
}
```


变动 `api/mod.rs`

```rust
pub mod custom_middleware;
pub mod hello_world;
pub mod middleware_message;
pub mod mirror_body_json;
pub mod mirror_body_string;
pub mod mirror_custom_header;
pub mod mirror_user_agent;
pub mod path_variable;
pub mod query_params;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use custom_middleware::custom_middleware;
use hello_world::hello_world;
use middleware_message::middleware_message;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use mirror_custom_header::mirror_custom_header;
use mirror_user_agent::mirror_user_agent;
use path_variable::{hard_coded_path, path_variable};
use query_params::query_params;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

pub fn create_routes() -> Router {
    let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any);

    let shared_data = SharedData {
        message: "Hello from shared data".to_owned(),
    };

    Router::new()
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variable/:id", get(path_variable))
        .route("/path_variable/15", get(hard_coded_path))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .route("/middleware_message", get(middleware_message))
        .layer(Extension(shared_data))
        .layer(cors)
        .route("/custom_middleware", get(custom_middleware))
}
```

此时会出现这个问题

```shell
the trait bound `fn(Extension<HeaderMessage>) -> impl Future<Output = std::string::String> {custom_middleware}: Handler<_, _, _>` is not satisfied
the trait `Handler<T, S, B2>` is implemented for `Layered<L, H, T, S, B, B2>`
rustc for full compiler diagnostic
mod.rs(50, 38): required by a bound introduced by this call
method_routing.rs(145, 16): required by a bound in `axum::routing::get`
```

```shell
error[E0277]: the trait bound `fn(Extension<HeaderMessage>) -> impl Future<Output = std::string::String> {custom_middleware}: Handler<_, _, _>` is not satisfied
   --> src/api/mod.rs:50:42
    |
50  |         .route("/custom_middleware", get(custom_middleware))
    |                                      --- ^^^^^^^^^^^^^^^^^ the trait `Handler<_, _, _>` is not implemented for fn item `fn(Extension<HeaderMessage>) -> impl Future<Output = std::string::String> {custom_middleware}`
    |                                      |
    |                                      required by a bound introduced by this call
    |
    = help: the trait `Handler<T, S, B2>` is implemented for `Layered<L, H, T, S, B, B2>`
note: required by a bound in `axum::routing::get`
   --> /Users/.cargo/registry/src/rsproxy.cn-8f6827c7555bfaf8/axum-0.6.4/src/routing/method_routing.rs:403:1
    |
403 | top_level_handler_fn!(get, GET);
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `axum::routing::get`
    = note: this error originates in the macro `top_level_handler_fn` (in Nightly builds, run with -Z macro-backtrace for more info)
```

解决办法是加 `Clone` trait

```rust
use axum::Extension;

#[derive(Clone)]
pub struct HeaderMessage(String);

pub async fn custom_middleware(Extension(message): Extension<HeaderMessage>) -> String {
    message.0
}
```

现在访问 http://localhost:3000/custom_middleware 发现服务端 500 错误

```shell
Missing request extension: Extension of type `routing::api::custom_middleware::HeaderMessage` was not found. Perhaps you forgot to add it? See `axum::Extension`.
```

因为缺少了 `axum::Extension` 新建一个 middleware 文件 `set_custom_middleware.rs`

添加自定义 middleware 
- https://docs.rs/axum/latest/axum/middleware/index.html#writing-middleware
- https://docs.rs/axum/latest/axum/middleware/fn.from_fn.html

Create a middleware from an async function.

from_fn requires the function given to

1. Be an async fn.
2. Take one or more [extractors](https://docs.rs/axum/latest/axum/extract/trait.FromRequest.html) as the first arguments.
3. Take `[Next<B>](https://docs.rs/axum/latest/axum/middleware/struct.Next.html)` as the final argument.
4. Return something that implements [IntoResponse](https://docs.rs/axum/latest/axum/response/trait.IntoResponse.html).

Note that this function doesn’t support extracting [State](https://docs.rs/axum/latest/axum/extract/struct.State.html). For that, use [from_fn_with_state](https://docs.rs/axum/latest/axum/middleware/fn.from_fn_with_state.html).

```rust
use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use super::custom_middleware::HeaderMessage;

pub async fn set_custom_middleware<B>(
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    // do something with `request`...
    let headers = request.headers();
    let message = headers
        .get("message")
        .ok_or_else(|| StatusCode::BAD_REQUEST)?;
    let message = message.to_str().map_err(|_| StatusCode::BAD_REQUEST)?;
    request
        .extensions_mut()
        .insert(HeaderMessage(message.to_owned()));

    let response = next.run(request).await;

    // do something with `response`...

    Ok(response)
}
```

常见错误

```shell
error[E0502]: cannot borrow `request` as mutable because it is also borrowed as immutable
  --> src/api/set_custom_middleware.rs:19:5
   |
14 |       let headers = request.headers();
   |                     ----------------- immutable borrow occurs here
...
19 | /     request
20 | |         .extensions_mut()
   | |_________________________^ mutable borrow occurs here
21 |           .insert(HeaderMessage(message.to_owned()));
   |                                 ------------------ immutable borrow later used here
```

修改为

```rust
use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use super::custom_middleware::HeaderMessage;

pub async fn set_custom_middleware<B>(
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    // do something with `request`...
    let headers = request.headers();
    let message = headers
        .get("message")
        .ok_or_else(|| StatusCode::BAD_REQUEST)?;
    let message = message
        .to_str()
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .to_owned();
    request.extensions_mut().insert(HeaderMessage(message));

    let response = next.run(request).await;

    // do something with `response`...

    Ok(response)
}
```

需要将自定义中间价只作用于该路由，需要将该路由放到最上面，否则上面的所有路由 header 都需要有 message

```rust
pub mod custom_middleware;
pub mod hello_world;
pub mod middleware_message;
pub mod mirror_body_json;
pub mod mirror_body_string;
pub mod mirror_custom_header;
pub mod mirror_user_agent;
pub mod path_variable;
pub mod query_params;
pub mod set_custom_middleware;

use axum::{
    middleware,
    routing::{get, post},
    Extension, Router,
};
use custom_middleware::custom_middleware;
use hello_world::hello_world;
use middleware_message::middleware_message;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use mirror_custom_header::mirror_custom_header;
use mirror_user_agent::mirror_user_agent;
use path_variable::{hard_coded_path, path_variable};
use query_params::query_params;
use set_custom_middleware::set_custom_middleware;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

pub fn create_routes() -> Router {
    let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any);

    let shared_data = SharedData {
        message: "Hello from shared data".to_owned(),
    };

    Router::new()
        .route("/custom_middleware", get(custom_middleware))
        .route_layer(middleware::from_fn(set_custom_middleware))
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variable/:id", get(path_variable))
        .route("/path_variable/15", get(hard_coded_path))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .route("/middleware_message", get(middleware_message))
        .layer(Extension(shared_data))
        .layer(cors)
}
```


[代码变动](https://github.com/CusterFun/rust-exercises/commit/a23c4ff5d0949df242341b66c5541d9668fbe3d2)

## 14. HTTP Status Codes

```HTTPie
http GET http://localhost:3000/always_errors \
  Accept:'*/*' \
  User-Agent:'Thunder Client (https://www.thunderclient.com)'
```

新建文件 `api/always_errors.rs`

```rust
use axum::http::StatusCode;

pub async fn always_errors() -> Result<(), StatusCode> {
    // Ok(())
    Err(StatusCode::IM_A_TEAPOT)
}
```

<details><summary>变动 `api/mod.rs`</summary>

```rust
pub mod always_errors;
pub mod custom_middleware;
pub mod hello_world;
pub mod middleware_message;
pub mod mirror_body_json;
pub mod mirror_body_string;
pub mod mirror_custom_header;
pub mod mirror_user_agent;
pub mod path_variable;
pub mod query_params;
pub mod set_custom_middleware;

use always_errors::always_errors;
use axum::{
    middleware,
    routing::{get, post},
    Extension, Router,
};
use custom_middleware::custom_middleware;
use hello_world::hello_world;
use middleware_message::middleware_message;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use mirror_custom_header::mirror_custom_header;
use mirror_user_agent::mirror_user_agent;
use path_variable::{hard_coded_path, path_variable};
use query_params::query_params;
use set_custom_middleware::set_custom_middleware;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

pub fn create_routes() -> Router {
    let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any);

    let shared_data = SharedData {
        message: "Hello from shared data".to_owned(),
    };

    Router::new()
        .route("/custom_middleware", get(custom_middleware))
        .route_layer(middleware::from_fn(set_custom_middleware))
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variable/:id", get(path_variable))
        .route("/path_variable/15", get(hard_coded_path))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .route("/middleware_message", get(middleware_message))
        .layer(Extension(shared_data))
        .layer(cors)
        .route("/always_errors", get(always_errors))
}
```
</details>

[代码变动](https://github.com/CusterFun/rust-exercises/commit/95eae93b7387d1dcd989765a4a3f286e4f1984d7)

## 15. 200 HTTP Codes

文档 [https://docs.rs/axum/latest/axum/response/index.html](https://docs.rs/axum/latest/axum/response/index.html#returning-different-response-types)


```HTTPie
http GET localhost:3000/returns_201 \
  Accept:'*/*' \
  User-Agent:'Thunder Client (https://www.thunderclient.com)'
```

新建文件 `api/returns_201.rs`

```rust
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub async fn returns_201() -> Response {
    (StatusCode::CREATED, ()).into_response()
}
```

<details><summary>变动 `api/mod.rs`</summary>

```rust
pub mod always_errors;
pub mod custom_middleware;
pub mod hello_world;
pub mod middleware_message;
pub mod mirror_body_json;
pub mod mirror_body_string;
pub mod mirror_custom_header;
pub mod mirror_user_agent;
pub mod path_variable;
pub mod query_params;
pub mod returns_201;
pub mod set_custom_middleware;

use always_errors::always_errors;
use axum::{
    middleware,
    routing::{get, post},
    Extension, Router,
};
use custom_middleware::custom_middleware;
use hello_world::hello_world;
use middleware_message::middleware_message;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use mirror_custom_header::mirror_custom_header;
use mirror_user_agent::mirror_user_agent;
use path_variable::{hard_coded_path, path_variable};
use query_params::query_params;
use returns_201::returns_201;
use set_custom_middleware::set_custom_middleware;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

pub fn create_routes() -> Router {
    let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any);

    let shared_data = SharedData {
        message: "Hello from shared data".to_owned(),
    };

    Router::new()
        .route("/custom_middleware", get(custom_middleware))
        .route_layer(middleware::from_fn(set_custom_middleware))
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variable/:id", get(path_variable))
        .route("/path_variable/15", get(hard_coded_path))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .route("/middleware_message", get(middleware_message))
        .layer(Extension(shared_data))
        .layer(cors)
        .route("/always_errors", get(always_errors))
        .route("/returns_201", get(returns_201))
}
```
</details>

[代码变动](https://github.com/CusterFun/rust-exercises/commit/c76d34c755c112ea9923bf4c58b5b9ed78475fd1)

## 16. Returning JSON

```HTTPie
http GET http://localhost:3000/get_json \
  Accept:'*/*' \
  User-Agent:'Thunder Client (https://www.thunderclient.com)'
```

新建文件 `api/get_json.rs`

```rust
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Data {
    message: String,
    count: u32,
    username: String,
}

pub async fn get_json() -> Json<Data> {
    let data = Data {
        message: "I'm in data".to_owned(),
        count: 4975,
        username: "I'm Custer".to_owned(),
    };
    Json(data)
}
```

<details><summary>变动 `api/mod.rs`</summary>

```rust
pub mod always_errors;
pub mod custom_middleware;
pub mod get_json;
pub mod hello_world;
pub mod middleware_message;
pub mod mirror_body_json;
pub mod mirror_body_string;
pub mod mirror_custom_header;
pub mod mirror_user_agent;
pub mod path_variable;
pub mod query_params;
pub mod returns_201;
pub mod set_custom_middleware;

use always_errors::always_errors;
use axum::{
    middleware,
    routing::{get, post},
    Extension, Router,
};
use custom_middleware::custom_middleware;
use get_json::get_json;
use hello_world::hello_world;
use middleware_message::middleware_message;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use mirror_custom_header::mirror_custom_header;
use mirror_user_agent::mirror_user_agent;
use path_variable::{hard_coded_path, path_variable};
use query_params::query_params;
use returns_201::returns_201;
use set_custom_middleware::set_custom_middleware;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

pub fn create_routes() -> Router {
    let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any);

    let shared_data = SharedData {
        message: "Hello from shared data".to_owned(),
    };

    Router::new()
        .route("/custom_middleware", get(custom_middleware))
        .route_layer(middleware::from_fn(set_custom_middleware))
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variable/:id", get(path_variable))
        .route("/path_variable/15", get(hard_coded_path))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .route("/middleware_message", get(middleware_message))
        .layer(Extension(shared_data))
        .layer(cors)
        .route("/always_errors", get(always_errors))
        .route("/returns_201", get(returns_201))
        .route("/get_json", get(get_json))
}
```
</details>


[代码变动](https://github.com/CusterFun/rust-exercises/commit/7938cdca49c6d0e4401f1d53168dd896429460b7)

## 17. Validating with Serde

Validate incoming data
- [17. Validating JSON with Serde](#17-validating-with--serde)
- [22. Custom Extractor with Validation](#22-custom-extractors)


```HTTPie
echo '{
  "username": "Custer",
  "password": "1234"
}' |  \
  http POST http://localhost:3000/validate_data \
  Accept:'*/*' \
  Content-Type:application/json \
  User-Agent:'Thunder Client (https://www.thunderclient.com)'
```

新建文件 `api/validate_data.rs`

```rust
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RequestUser {
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize)]
pub struct ResponseUser {
    code: i32,
    username: String,
    message: String,
}

pub async fn validate_data(Json(body): Json<RequestUser>) -> Json<ResponseUser> {
    Json(ResponseUser {
        code: 0,
        username: body.username.unwrap_or_default(),
        message: "success".to_string(),
    })
}
```

<details><summary>变动 `api/mod.rs`</summary>

```rust
pub mod always_errors;
pub mod custom_middleware;
pub mod get_json;
pub mod hello_world;
pub mod middleware_message;
pub mod mirror_body_json;
pub mod mirror_body_string;
pub mod mirror_custom_header;
pub mod mirror_user_agent;
pub mod path_variable;
pub mod query_params;
pub mod returns_201;
pub mod set_custom_middleware;
pub mod validate_data;

use always_errors::always_errors;
use axum::{
    middleware,
    routing::{get, post},
    Extension, Router,
};
use custom_middleware::custom_middleware;
use get_json::get_json;
use hello_world::hello_world;
use middleware_message::middleware_message;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use mirror_custom_header::mirror_custom_header;
use mirror_user_agent::mirror_user_agent;
use path_variable::{hard_coded_path, path_variable};
use query_params::query_params;
use returns_201::returns_201;
use set_custom_middleware::set_custom_middleware;
use tower_http::cors::{Any, CorsLayer};
use validate_data::validate_data;

#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

pub fn create_routes() -> Router {
    let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any);

    let shared_data = SharedData {
        message: "Hello from shared data".to_owned(),
    };

    Router::new()
        .route("/custom_middleware", get(custom_middleware))
        .route_layer(middleware::from_fn(set_custom_middleware))
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variable/:id", get(path_variable))
        .route("/path_variable/15", get(hard_coded_path))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .route("/middleware_message", get(middleware_message))
        .layer(Extension(shared_data))
        .layer(cors)
        .route("/always_errors", get(always_errors))
        .route("/returns_201", get(returns_201))
        .route("/get_json", get(get_json))
        .route("/validate_data", post(validate_data))
}
```
</details>


[代码变动](https://github.com/CusterFun/rust-exercises/commit/f76ea47a4fc1275b926a7707222a4770637ae3de)

## 18. Creating a Database

初始化 seaorm-axum 项目，新建 postgres 数据库

[代码变动](https://github.com/CusterFun/rust-exercises/commit/8fe0dbf7f039e7529298e29e23aec21fef196980)

## 19. SeaORM

https://www.sea-ql.org/SeaORM/docs/install-and-config/database-and-async-runtime/


```shell
cargo add sea-orm -F sqlx-postgres -F runtime-tokio-rustls
cargo add tokio -F macros -F rt-multi-thread
cargo add axum -F headers
cargo add tower-http -F cors  
cargo add serde -F derive 
```

## 20. Connecting SeaORM to the Database

```shell
touch .env
```

写入

```text
DATABASE_URL=postgres://postgres:postgres@localhost/user_task
```

```shell
cargo add dotenvy
cargo add dotenvy_macro
```

新建文件 `main.rs`

```rust
use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use seaorm_axum::run;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_uri = dotenv!("DATABASE_URL");
    run(database_uri).await;
}
```

新建文件 `lib.rs`

```rust
use sea_orm::Database;

pub async fn run(database_uri: &str) {
    let database = Database::connect(database_uri).await;
}
```

[代码变动](https://github.com/CusterFun/rust-exercises/commit/f82ed791b1a232ab56e304c2878599f62523a13f)

## 21. Generating SeaORM Models
https://www.sea-ql.org/SeaORM/docs/generate-entity/sea-orm-cli/

```shell
❯ cargo install sea-orm-cli
❯ sea-orm-cli -h 
❯ sea-orm-cli generate -h
❯ sea-orm-cli generate entity -h
❯ sea-orm-cli generate entity -o src/database
❯ sea-orm-cli generate entity -o src/databases                   
Generating tasks.rs
    > Column `id`: i32, auto_increment, not_null
    > Column `priority`: Option<String>
    > Column `title`: String, not_null
    > Column `completed_at`: Option<DateTimeWithTimeZone>
    > Column `description`: Option<String>
    > Column `deleted_at`: Option<DateTimeWithTimeZone>
    > Column `user_id`: Option<i32>
    > Column `is_default`: Option<bool>
Generating users.rs
    > Column `id`: i32, auto_increment, not_null
    > Column `username`: String, not_null, unique
    > Column `password`: String, not_null
    > Column `deleted_at`: Option<DateTimeWithTimeZone>
    > Column `token`: Option<String>
```

## 22. Custom Extractors

Validate incoming data
- [17. Validating JSON with Serde](#17-validating-with--serde)
- [22. Custom Extractor with Validation](#22-custom-extractors)


新建文件夹 `api` 和 `api/mod.rs`

```rust
pub mod custom_json_extractor;

pub use custom_json_extractor::custom_json_extractor;
```

新建文件 `api/custom_json_extractor.rs`

```rust
pub async fn custom_json_extractor() {}
```

新建文件 `router.rs`

```rust
use axum::{routing::post, Router};
use sea_orm::DatabaseConnection;

use crate::api::custom_json_extractor;

pub async fn create_routes(_database: DatabaseConnection) -> Router {
    Router::new().route("/custom_json_extractor", post(custom_json_extractor))
}
```

修改 `lib.rs`

```rust
pub mod api;
mod router;

use sea_orm::Database;

pub async fn run(database_uri: &str) {
    let database = Database::connect(database_uri).await.unwrap();
    let app = router::create_routes(database).await;

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

文档 Example 示例 https://docs.rs/axum/latest/axum/extract/trait.FromRequest.html#what-is-the-b-type-parameter

```rust
use axum::{
    async_trait,
    extract::FromRequest,
    http::{self, Request},
};

struct MyExtractor;

#[async_trait]
impl<S, B> FromRequest<S, B> for MyExtractor
where
    // these bounds are required by `async_trait`
    B: Send + 'static,
    S: Send + Sync,
{
    type Rejection = http::StatusCode;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        // ...
    }
}
```

文档 https://docs.rs/axum/latest/axum/extract/trait.FromRequest.html#impl-FromRequest%3CS%2C%20B%2C%20ViaRequest%3E-for-Json%3CT%3E

```rust
impl<T, S, B> FromRequest<S, B> for Json<T>
where
    T: DeserializeOwned,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = JsonRejection;
```

此时 `custom_json_extractor.rs` 文件

```rust
use axum::{
    async_trait,
    body::HttpBody,
    extract::FromRequest,
    http::{Request, StatusCode},
    BoxError, Json, RequestExt,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RequestUser {
    pub username: String,
    pub password: String,
}

#[async_trait]
impl<S, B> FromRequest<S, B> for RequestUser
where
    S: Send + Sync,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = (StatusCode, String);
    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(user) = req
            .extract::<Json<RequestUser>, _>()
            .await
            .map_err(|err| (StatusCode::BAD_REQUEST, format!("{}", err)))?;

        Ok(user)
    }
}

pub async fn custom_json_extractor(user: RequestUser) -> Json<RequestUser> {
    Json(user)
}
```

访问

```shell
curl -X POST \
  'localhost:3000/custom_json_extractor' \
  --header 'Accept: */*' \
  --header 'User-Agent: Thunder Client (https://www.thunderclient.com)' \
  --header 'Content-Type: application/json' \
  --data-raw '{
  "username": "Custer",
  "password": "1234"
}'
```

添加 [`validator` crate](https://crates.io/crates/validator)

```shell
cargo add validator -F derive
```

修改 `custom_json_extractor.rs`

```rust
use axum::{
    async_trait,
    body::HttpBody,
    extract::FromRequest,
    http::{Request, StatusCode},
    BoxError, Json, RequestExt,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct RequestUser {
    #[validate(email(message = "must be a valid email"))]
    pub username: String,
    #[validate(length(min = 6, message = "must be at least 6 characters"))]
    pub password: String,
}

#[async_trait]
impl<S, B> FromRequest<S, B> for RequestUser
where
    S: Send + Sync,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = (StatusCode, String);
    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(user) = req
            .extract::<Json<RequestUser>, _>()
            .await
            .map_err(|err| (StatusCode::BAD_REQUEST, format!("{}", err)))?;

        if let Err(err) = user.validate() {
            return Err((StatusCode::BAD_REQUEST, err.to_string()));
        }
        Ok(user)
    }
}

pub async fn custom_json_extractor(user: RequestUser) -> Json<RequestUser> {
    Json(user)
}
```

此时再访问 

```shell
curl -X POST \
  'localhost:3000/custom_json_extractor' \
  --header 'Accept: */*' \
  --header 'User-Agent: Thunder Client (https://www.thunderclient.com)' \
  --header 'Content-Type: application/json' \
  --data-raw '{
  "username": "Custer",
  "password": "1234"
}'
```

会报错

```shell
username: must be a valid email
password: must be at least 6 characters
```

访问

```shell
curl -X POST \
  'localhost:3000/custom_json_extractor' \
  --header 'Accept: */*' \
  --header 'User-Agent: Thunder Client (https://www.thunderclient.com)' \
  --header 'Content-Type: application/json' \
  --data-raw '{
  "username": "Custer@email.com",
  "password": "123456"
}'
```

实现自定义提取器，可以在 `handler` 函数处理前进行预处理

[代码变动](https://github.com/CusterFun/rust-exercises/commit/d8f6c6c2cee50c97bd4164501fb8985e6be5cbac)

### 延伸 验证失败，返回 200 和自定义 json 数据

```rust
use axum::{
    async_trait,
    body::HttpBody,
    extract::FromRequest,
    http::{Request, StatusCode},
    BoxError, Json, RequestExt,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct RequestUser {
    #[validate(email(message = "must be a valid email"))]
    pub username: String,
    #[validate(length(min = 6, message = "must be at least 6 characters"))]
    pub password: String,
}

#[derive(Serialize)]
pub struct ResponseErr {
    pub msg: String,
    pub code: i8,
}

#[async_trait]
impl<S, B> FromRequest<S, B> for RequestUser
where
    S: Send + Sync,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = Json<ResponseErr>;
    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(user) =
            req.extract::<Json<RequestUser>, _>()
                .await
                .map_err(|err| ResponseErr {
                    msg: err.to_string(),
                    code: -1,
                })?;

        if let Err(err) = user.validate() {
            let res = ResponseErr {
                msg: err.to_string(),
                code: -1,
            };
            return Err(Json(res));
        }
        Ok(user)
    }
}

pub async fn custom_json_extractor(user: RequestUser) -> Json<RequestUser> {
    Json(user)
}
```

此时访问 

```shell
curl -X POST \
  'localhost:3000/custom_json_extractor' \
  --header 'Accept: */*' \
  --header 'User-Agent: Thunder Client (https://www.thunderclient.com)' \
  --header 'Content-Type: application/json' \
  --data-raw '{
  "username": "Custer.com",
  "password": "1234"
}'
```

可以看到错误为 200

```json
{
  "msg": "username: must be a valid email\npassword: must be at least 6 characters",
  "code": -1
}
```

[代码变动](https://github.com/CusterFun/rust-exercises/commit/d91decf877aa9ef20db343801acc033693986eea#diff-509be60ef29964b4df481a6193b1821c6dd87cb6ab6afb5e61460d7913cd7b27)

## 23. Passing Data to Handlers

```HTTPie

```

新建文件 `api/.rs`

```rust

```

<details><summary>变动 `api/mod.rs`</summary>

```rust

```
</details>

[代码变动](

## 24. Inserting to the Database

```HTTPie

```

新建文件 `api/.rs`

```rust

```

<details><summary>变动 `api/mod.rs`</summary>

```rust

```
</details>

[代码变动](

## 25. Selecting One Item from the Database

```HTTPie

```

新建文件 `api/.rs`

```rust

```

<details><summary>变动 `api/mod.rs`</summary>

```rust

```
</details>

[代码变动](

## 26. Get all from the Database

```HTTPie

```

新建文件 `api/.rs`

```rust

```

<details><summary>变动 `api/mod.rs`</summary>

```rust

```
</details>

[代码变动](

## 27. Using SeaORM filters

```HTTPie

```

新建文件 `api/.rs`

```rust

```

<details><summary>变动 `api/mod.rs`</summary>

```rust

```
</details>

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