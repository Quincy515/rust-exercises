- [1.Introduce the project](#1introduce-the-project)
- [2.Hello World](#2hello-world)
- [3.Create User](#3create-user)
  - [nest router](#nest-router)
  - [Response user](#response-user)
  - [Database URL](#database-url)
  - [Creating user](#creating-user)
  - [Json Web Token](#json-web-token)
  - [Hash password](#hash-password)

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
export DATABASE_URL=postgres://postgres:postgres@localhost/user_task
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
export API_URI=0.0.0.0
export JWT_SECRET=jwt_secret
export DATABASE_URL=postgres://postgres:postgres@localhost/user_task
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

`curl` 请求

```shell
curl -X GET 'localhost:3000' 
```

## 3.Create User
### nest router
https://docs.rs/axum/latest/axum/routing/struct.Router.html#method.nest

修改 `router.rs`

```rust
use axum::{routing::get, Router};

use crate::api::hello::hello;

pub async fn create_router() -> Router {
    let user_routes = Router::new().route("/", get(|| async {}));
    
    let task_routes = Router::new().route("/", get(|| async {}));

    let api_routes = Router::new()
        .nest("/users", user_routes)
        .nest("/tasks", task_routes);

    Router::new()
        .route("/", get(hello))
        .nest("/api/v1/", api_routes)
}
```
### Response user
新建外部的 `crate` 保存数据模型和请求返回的数据结构，为了以后可以和 `yew` 通用

```shell
cargo new --lib types
cd types
cargo add serde -F derive
cargo add serde_json
```

在 `types` 项目中新建 `user.rs` 并修改 `lib.rs`

```rust
pub mod user;
```

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ResponseUser {
    pub data: Option<bool>,
}
```

然后在 `server` 项目中使用

```toml
types = { path="../types" }
```

新建文件夹 `api/users` 和文件 `api/users/mod.rs` 、 `api/users/create_user.rs`

```rust
use axum::Json;
use types::user::ResponseUser;

pub async fn create_user() -> Json<ResponseUser> {
    let response = ResponseUser { data: Some(true) };
    Json(response)
}
```

<details><summary>修改 api/users/mod.rs </summary>

```rust
pub mod create_user;
```
</details>

<details><summary>修改 router.rs </summary>

```rust
use axum::{
    routing::{get, post},
    Router,
};

use crate::api::{hello::hello, users::create_user::create_user};

pub async fn create_router() -> Router {
    let user_routes = Router::new().route("/", post(create_user));

    let task_routes = Router::new().route("/", get(|| async {}));

    let api_routes = Router::new()
        .nest("/users", user_routes)
        .nest("/tasks", task_routes);

    Router::new()
        .route("/", get(hello))
        .nest("/api/v1/", api_routes)
}
```
</details>

此时访问 curl

```shell
curl -X POST 'localhost:3000/api/v1/users'
```

返回

```json
{
  "data": true
}
```

### Database URL

在 `server` 目录下运行 

```shell
sea-orm-cli generate entity -l -o ../entity/src
cd ../entity
cargo init
cargo add sea-orm
cargo add serde -F derive
```

然后在 `server/Cargo.toml` 中添加 

```toml
entity = {path = "../entity"}
```

### Creating user

增加创建用的返回结构体，修改 `types/src/user.rs`

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ResponseDataUser {
    pub data: ResponseUser,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseUser {
    pub id: i32,
    pub username: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestCreateUser {
    pub username: String,
    pub password: String,
}
```
新建 `util/mod.rs` 和 `util/app_error.rs`

```rust
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub struct AppError {
    code: StatusCode,
    message: String,
}

impl AppError {
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            self.code,
            Json(ErrorResponse {
                error: self.message.clone(),
            }),
        )
            .into_response()
    }
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
}
```

新建 `app_state.rs`

在 `axum` 中使用 `state` 文档 https://docs.rs/axum/latest/axum/extract/struct.State.html

```rust
use axum::extract::FromRef;
use sea_orm::DatabaseConnection;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: DatabaseConnection,
}
```

在 `router.rs` 中传递 `state` 

```rust
use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{
    api::{hello::hello, users::create_user::create_user},
    app_state::AppState,
};

pub async fn create_router(app_state: AppState) -> Router {
    let user_routes = Router::new()
        .route("/", post(create_user))
        .with_state(app_state);

    let task_routes = Router::new().route("/", get(|| async {}));

    let api_routes = Router::new()
        .nest("/users", user_routes)
        .nest("/tasks", task_routes);

    Router::new()
        .route("/", get(hello))
        .nest("/api/v1/", api_routes)
}
```

修改 `server/src/api/users/create_user.rs`

```rust
use axum::{extract::State, http::StatusCode, Json};
use entity::users;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use types::user::{RequestCreateUser, ResponseDataUser, ResponseUser};

use crate::util::app_error::AppError;

pub async fn create_user(
    State(db): State<DatabaseConnection>,
    Json(request_user): Json<RequestCreateUser>,
) -> Result<Json<ResponseDataUser>, AppError> {
    let mut new_user = users::ActiveModel {
        ..Default::default()
    };
    new_user.username = Set(request_user.username);
    new_user.password = Set(request_user.password);
    let user = new_user.save(&db).await.map_err(|error| {
        eprintln!("Error creating user: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
    })?;

    Ok(Json(ResponseDataUser {
        data: ResponseUser {
            id: user.id.unwrap(),
            username: user.username.unwrap(),
            token: user.token.unwrap().unwrap_or_default(),
        },
    }))
}
```

[Get an owned value of the ActiveValue](https://docs.rs/sea-orm/latest/sea_orm/entity/enum.ActiveValue.html#method.unwrap)

也可以用 [`try_into_model`](https://docs.rs/sea-orm/latest/sea_orm/entity/trait.TryIntoModel.html#tymethod.try_into_model) 将 [`ActiveModel` 转换为 `Model` ](https://www.sea-ql.org/SeaORM/docs/basic-crud/insert/#convert-activemodel-back-to-model)

```rust
use axum::{extract::State, http::StatusCode, Json};
use entity::users;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TryIntoModel};
use types::user::{RequestCreateUser, ResponseDataUser, ResponseUser};

use crate::util::app_error::AppError;

pub async fn create_user(
    State(db): State<DatabaseConnection>,
    Json(request_user): Json<RequestCreateUser>,
) -> Result<Json<ResponseDataUser>, AppError> {
    let mut new_user = users::ActiveModel {
        ..Default::default()
    };
    new_user.username = Set(request_user.username);
    new_user.password = Set(request_user.password);
    let user = new_user
        .save(&db)
        .await
        .map_err(|error| {
            eprintln!("Error creating user: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
        })?
        .try_into_model()
        .map_err(|err| {
            eprintln!("Error converting user back into model: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
        })?;

    Ok(Json(ResponseDataUser {
        data: ResponseUser {
            id: user.id,
            username: user.username,
            token: user.token.unwrap_or_default(),
        },
    }))
}
```

[代码变动](https://github.com/CusterFun/rust-exercises/commit/f1abc92839b14a80fdb4d35555eccbdcad3b26a1#diff-ee6af3e44180d400670d53d2b574ab6e4526b5a568a6d40c244b0db1b7f4f0bb)

使用 curl 测试

```shell
curl -X POST \
  'http://localhost:3000/api/v1/users' \
  --header 'Accept: */*' \
  --header 'User-Agent: Thunder Client (https://www.thunderclient.com)' \
  --header 'Content-Type: application/json' \
  --data-raw '{
  "username": "Custer",
  "password": "1234"
}'
```

返回 Json

```json
{
  "data": {
    "id": 9,
    "username": "Custer3",
    "token": ""
  }
}
```

[ 代码变动 ](https://github.com/CusterFun/rust-exercises/commit/f1e97ca1c79960f98463097bd633e18978d9752b)


### Json Web Token

修改 `app_state.rs` 增加 `jwt_secret` 
```rust
use axum::extract::FromRef;
use sea_orm::DatabaseConnection;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub jwt_secret: String,
}
```
在 `main.rs` 中获取 `.env` 中设置的 `jwt_secret` 并保存到 `AppState` 中

```rust
use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use sea_orm::Database;
use server::{app_state::AppState, run};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url: &str = dotenv!("DATABASE_URL");
    let jwt_secret: String = dotenv!("JWT_SECRET").to_owned();
    let db = match Database::connect(database_url).await {
        Ok(db) => db,
        Err(err) => {
            eprintln!("Error connecting to the databases: {:?}", err);
            std::process::exit(1);
        }
    };
    let app_state = AppState { db, jwt_secret };
    run(app_state).await;
}
```

在 `util` 目录中新建 `jwt.rs` 来处理 `token` 的创建和认证

```rust
use axum::http::StatusCode;
use chrono::Duration;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use super::app_error::AppError;

#[derive(Serialize, Deserialize, Debug)]
struct Claims {
    exp: usize,
}

pub fn create_token(secret: &str) -> Result<String, AppError> {
    let now = chrono::Utc::now();
    let expires_at = now + Duration::hours(1);
    let exp = expires_at.timestamp() as usize;
    let claims = Claims { exp };
    let token_header = Header::default();
    let key = EncodingKey::from_secret(secret.as_bytes());
    encode(&token_header, &claims, &key).map_err(|err| {
        eprintln!("Error creating token: {err:?}");
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "There was an error creating the token",
        )
    })
}
```

这样就可以在 `api/create_user.rs` 中使用

```rust
use axum::{extract::State, http::StatusCode, Json};
use entity::users;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TryIntoModel};
use types::user::{RequestCreateUser, ResponseDataUser, ResponseUser};

use crate::util::{app_error::AppError, jwt::create_token};

pub async fn create_user(
    State(db): State<DatabaseConnection>,
    State(secret): State<String>,
    Json(request_user): Json<RequestCreateUser>,
) -> Result<Json<ResponseDataUser>, AppError> {
    let mut new_user = users::ActiveModel {
        ..Default::default()
    };
    new_user.username = Set(request_user.username);
    new_user.password = Set(request_user.password);
    new_user.token = Set(Some(create_token(&secret)?));
    let user = new_user
        .save(&db)
        .await
        .map_err(|error| {
            eprintln!("Error creating user: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
        })?
        .try_into_model()
        .map_err(|err| {
            eprintln!("Error converting user back into model: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
        })?;

    Ok(Json(ResponseDataUser {
        data: ResponseUser {
            id: user.id,
            username: user.username,
            token: user.token.unwrap_or_default(),
        },
    }))
}
```

使用 curl 创建新用户

```shell
curl -X POST \
  'http://localhost:3000/api/v1/users' \
  --header 'Accept: */*' \
  --header 'User-Agent: Thunder Client (https://www.thunderclient.com)' \
  --header 'Content-Type: application/json' \
  --data-raw '{
  "username": "Custer7",
  "password": "1234"
}'
```

返回 Json

```json
{
  "data": {
    "id": 15,
    "username": "Custer7",
    "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2NzYwODk2NTl9.0fbI5BOiXSa9tc27l2sT-TvCVyFgfSMbvH4bhNS8XGY"
  }
}
```

[代码变动](https://github.com/CusterFun/rust-exercises/commit/d58fe67504eb947b88d3a3958eeff8cf935b3798#diff-ee6af3e44180d400670d53d2b574ab6e4526b5a568a6d40c244b0db1b7f4f0bb)

### Hash password

首先给 `jwt` 的 `claims` 增加 `username` 

```rust
use axum::http::StatusCode;
use chrono::Duration;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use super::app_error::AppError;

#[derive(Serialize, Deserialize, Debug)]
struct Claims {
    exp: usize,
    username: String,
}

pub fn create_token(secret: &str, username: String) -> Result<String, AppError> {
    let now = chrono::Utc::now();
    let expires_at = now + Duration::hours(1);
    let exp = expires_at.timestamp() as usize;
    let claims = Claims { exp, username };
    let token_header = Header::default();
    let key = EncodingKey::from_secret(secret.as_bytes());
    encode(&token_header, &claims, &key).map_err(|err| {
        eprintln!("Error creating token: {err:?}");
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "There was an error creating the token",
        )
    })
}
```

然后增加 `password` 的 `bcrypt` 加密，新增文件 `util/hash.rs`

```rust
use axum::http::StatusCode;
use bcrypt::hash;

use crate::util::app_error::AppError;
pub fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password.as_bytes(), bcrypt::DEFAULT_COST).map_err(|err| {
        eprintln!("Error hashing password: {err:?}");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error securing password")
    })
}
```

然后在 `api/create_user.rs` 中加密 `password`

```rust
use axum::{extract::State, http::StatusCode, Json};
use entity::users;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TryIntoModel};
use types::user::{RequestCreateUser, ResponseDataUser, ResponseUser};

use crate::util::{app_error::AppError, hash::hash_password, jwt::create_token};

pub async fn create_user(
    State(db): State<DatabaseConnection>,
    State(secret): State<String>,
    Json(request_user): Json<RequestCreateUser>,
) -> Result<Json<ResponseDataUser>, AppError> {
    let mut new_user = users::ActiveModel {
        ..Default::default()
    };
    new_user.username = Set(request_user.username.clone());
    new_user.password = Set(hash_password(&request_user.password)?);
    new_user.token = Set(Some(create_token(&secret, request_user.username)?));
    let user = new_user
        .save(&db)
        .await
        .map_err(|error| {
            eprintln!("Error creating user: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
        })?
        .try_into_model()
        .map_err(|err| {
            eprintln!("Error converting user back into model: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
        })?;

    Ok(Json(ResponseDataUser {
        data: ResponseUser {
            id: user.id,
            username: user.username,
            token: user.token.unwrap_or_default(),
        },
    }))
}
```

[代码变动]()