[yew-tutorial](./yew-tutorial.md)

- [1.Introduce the project](#1introduce-the-project)
- [2.Hello World](#2hello-world)
- [3.Create User](#3create-user)
  - [nest router](#nest-router)
  - [Response user](#response-user)
  - [Database URL](#database-url)
  - [Creating user](#creating-user)
  - [Json Web Token](#json-web-token)
  - [Hash password](#hash-password)
- [4. Handing Duplicate Usernames](#4-handing-duplicate-usernames)
- [5. Singing in](#5-singing-in)
- [6. Middleware \& Logging out](#6-middleware--logging-out)
- [7. Creating Tasks](#7-creating-tasks)
  - [在 `handler` 中验证 `request`](#在-handler-中验证-request)
  - [在 `request` 提取器中验证，进入 `handler` 函数之前](#在-request-提取器中验证进入-handler-函数之前)
- [8. Getting All Tasks](#8-getting-all-tasks)
  - [创建新用户时添加默认task](#创建新用户时添加默认task)
  - [重构代码](#重构代码)
- [9. Get One Task](#9-get-one-task)
- [10. Update Tasks](#10-update-tasks)
  - [should be able to mark a task as completed](#should-be-able-to-mark-a-task-as-completed)
  - [should be able to mark a task as not completed](#should-be-able-to-mark-a-task-as-not-completed)
  - [should be able to update all fields in the task](#should-be-able-to-update-all-fields-in-the-task)
- [11. Soft Deleting Tasks](#11-soft-deleting-tasks)
- [12. Refactoring Queries](#12-refactoring-queries)
  - [简化 `create_task.rs`](#简化-create_taskrs)
  - [简化 `delete_task.rs`](#简化-delete_taskrs)
  - [简化 `save_active_task`](#简化-save_active_task)
  - [简化 `get_all_task.rs`](#简化-get_all_taskrs)
  - [简化 `get_one_task.rs`](#简化-get_one_taskrs)
  - [简化 `update_tasks.rs`](#简化-update_tasksrs)
  - [简化 `create_user.rs`](#简化-create_userrs)
  - [简化 `create_default_tasks_for_user`](#简化-create_default_tasks_for_user)
  - [简化 `login.rs`](#简化-loginrs)
  - [简化 `logout.rs`](#简化-logoutrs)

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
cargo add validator -F derive
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

[代码变动](https://github.com/CusterFun/rust-exercises/commit/445460cbdef25cad2aa321dacbcccdcadee72d6e#diff-ee6af3e44180d400670d53d2b574ab6e4526b5a568a6d40c244b0db1b7f4f0bb)

## 4. Handing Duplicate Usernames

目前创建重复的用户名，服务器会返回 500 错误

```shell
curl -X POST \
  'http://localhost:3000/api/v1/users' \
  --header 'Accept: */*' \
  --header 'User-Agent: Thunder Client (https://www.thunderclient.com)' \
  --header 'Content-Type: application/json' \
  --data-raw '{
  "username": "Custer8",
  "password": "1234"
}'

{
  "error": "Query Error: error returned from database: duplicate key value violates unique constraint \"users_username_key\""
}
```

可以捕获数据库返回的错误，也可以提前检查 `username` 是否存在

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
        .map_err(|err| {
            let error_message = err.to_string();
            if error_message
                .contains("duplicate key value violates unique constraint \"users_username_key\"")
            {
                AppError::new(
                    StatusCode::BAD_REQUEST,
                    "Username already taken, try again with a different user name",
                )
            } else {
                eprintln!("Error creating user: {:?}", &err);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
            }
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

这样发送重复的 `username` 注册新用户，就会返回自定义的 Json 数据

```rust
{
  "error": "Username already taken, try again with a different user name"
}
```

[代码变动](https://github.com/CusterFun/rust-exercises/commit/41fd05a18961897ec6b3701d6809898af96e737f#diff-ee6af3e44180d400670d53d2b574ab6e4526b5a568a6d40c244b0db1b7f4f0bb)

## 5. Singing in

新建用户登录的路由 `api/users/login.rs`

```rust
use axum::{extract::State, http::StatusCode, Json};
use entity::{prelude::*, users};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set, TryIntoModel,
};
use types::user::{RequestCreateUser, ResponseDataUser, ResponseUser};

use crate::util::{app_error::AppError, hash::verify_password, jwt::create_token};

pub async fn login(
    State(db): State<DatabaseConnection>,
    State(secret): State<String>,
    Json(request_user): Json<RequestCreateUser>,
) -> Result<Json<ResponseDataUser>, AppError> {
    let user = Users::find()
        .filter(users::Column::Username.eq(request_user.username.as_str()))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error getting user for logging in: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error loggin in")
        })?;

    if let Some(user) = user {
        if !verify_password(&request_user.password, &user.password)? {
            return Err(AppError::new(
                StatusCode::UNAUTHORIZED,
                "bad username or password",
            ));
        }
        let token = create_token(&secret, user.username.clone())?;
        let mut user = user.into_active_model();
        user.token = Set(Some(token));
        let user = user
            .save(&db)
            .await
            .map_err(|err| {
                eprintln!("Error saving user token: {:?}", err);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error saving user token")
            })?
            .try_into_model()
            .map_err(|err| {
                eprintln!("Error converting model to active model: {:?}", err);
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error converting model to active model",
                )
            })?;
        let response = ResponseUser {
            id: user.id,
            username: user.username,
            token: user.token.unwrap_or_default(),
        };

        Ok(Json(ResponseDataUser { data: response }))
    } else {
        Err(AppError::new(StatusCode::NOT_FOUND, "User not found"))
    }
}
```

这里需要验证密码，在 `util/hash.rs` 中添加 `verify_password` 函数

```rust
use axum::http::StatusCode;
use bcrypt::{hash, verify};

use crate::util::app_error::AppError;
pub fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password.as_bytes(), bcrypt::DEFAULT_COST).map_err(|err| {
        eprintln!("Error hashing password: {err:?}");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error securing password")
    })
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    verify(password, hash).map_err(|err| {
        eprintln!("Error verifying password: {err:?}");
        AppError::new(
            StatusCode::BAD_REQUEST,
            "There was an error verifying the password",
        )
    })
}
```

增加 `api/v1/users/login` 路由，修改 `router.rs`

<details>

```rust
use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    api::{
        hello::hello,
        users::{create_user::create_user, login::login},
    },
    app_state::AppState,
};

pub async fn create_router(app_state: AppState) -> Router {
    let user_routes = Router::new()
        .route("/", post(create_user))
        .route("/login", post(login))
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
</details>

发送 curl 请求

```shell
curl -X POST \
  'http://localhost:3000/api/v1/users/login' \
  --header 'Accept: */*' \
  --header 'User-Agent: Thunder Client (https://www.thunderclient.com)' \
  --header 'Content-Type: application/json' \
  --data-raw '{
  "username": "Custer8",
  "password": "1234"
}'
```

得到 Json 返回的数据

```json
{
  "data": {
    "id": 16,
    "username": "Custer8",
    "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2NzYxMjE3NzYsInVzZXJuYW1lIjoiQ3VzdGVyOCJ9.A2Hmu0-JCsiB0FKYpbGMSLf4WLFt05y4-WJHihXf_Sg"
  }
}
```
[代码变动](https://github.com/CusterFun/rust-exercises/commit/aba92eb508b688f7e4353736fa5d2575e2a2c6f4#diff-b5e833372dd39ee133868d12218c692a73c4ac09998fd7ec61ed61adc8e9c940)

## 6. Middleware & Logging out
首先在 `util/jwt.rs` 中添加验证 `token` 的方法 `validate_token`

```rust
use axum::http::StatusCode;
use chrono::Duration;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use super::app_error::AppError;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
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

pub fn validate_token(secret: &str, token: &str) -> Result<Claims, AppError> {
    let key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::new(Algorithm::HS256);
    decode::<Claims>(token, &key, &validation)
        .map_err(|err| match err.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken
            | jsonwebtoken::errors::ErrorKind::InvalidSignature
            | jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                AppError::new(StatusCode::UNAUTHORIZED, "not authenticated!")
            }
            _ => {
                eprintln!("Error validating token: {err:?}");
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "There was an error validating the token",
                )
            }
        })
        .map(|claim| claim.claims)
}
```

退出登录，需要在请求头中加 `token` 所以这里先添加中间件，新建文件夹 `middleware` 并新建文件 `middleware/require_authentication.rs`

```rust
use axum::{
    extract::State,
    http::{HeaderMap, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::util::{app_error::AppError, jwt::validate_token};
use entity::{prelude::*, users};

pub async fn require_authentication<T>(
    State(db): State<DatabaseConnection>,
    State(secret): State<String>,
    headers: HeaderMap,
    mut request: Request<T>,
    next: Next<T>,
) -> Result<Response, AppError> {
    let token = if let Some(token) = headers.get("x-token") {
        token.to_str().map_err(|err| {
            eprintln!("Error extracting token from headers: {err:?}");
            AppError::new(StatusCode::BAD_REQUEST, err.to_string())
        })?
    } else {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "Missing authentication token",
        ));
    };

    validate_token(&secret, token)?;

    let user = Users::find()
        .filter(users::Column::Token.eq(Some(token.to_owned())))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error getting user by token: {err:?}");
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was a problem getting your account",
            )
        })?;

    if let Some(user) = user {
        request.extensions_mut().insert(user);
    } else {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "You are not authorized for this",
        ));
    };
    Ok(next.run(request).await)
}
```

新增 `api/users/logout.rs` 文件，并将其添加到 `router.rs`

```rust
use axum::{extract::State, http::StatusCode, Extension};
use entity::users;
use sea_orm::{ActiveModelTrait, DatabaseConnection, IntoActiveModel, Set};

use crate::util::app_error::AppError;

pub async fn logout(
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<users::Model>,
) -> Result<StatusCode, AppError> {
    let mut user = user.into_active_model();
    user.token = Set(None);
    user.save(&db).await.map_err(|err| {
        eprintln!("Error removing token: {err:?}");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    })?;
    Ok(StatusCode::OK)
}
```

<details>

```rust
use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{
    api::{
        hello::hello,
        users::{create_user::create_user, login::login, logout::logout},
    },
    app_state::AppState,
    middleware::require_authentication::require_authentication,
};

pub async fn create_router(app_state: AppState) -> Router {
    let user_routes_auth =
        Router::new()
            .route("/logout", post(logout))
            .route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                require_authentication,
            ));
    let user_routes = Router::new()
        .route("/", post(create_user))
        .route("/login", post(login))
        .merge(user_routes_auth)
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
</details>

发送 `logout` 请求

```shell
curl -X POST \
  'http://localhost:3000/api/v1/users/logout' \
  --header 'Accept: */*' \
  --header 'User-Agent: Thunder Client (https://www.thunderclient.com)' \
  --header 'x-token: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2NzYxMzY1ODEsInVzZXJuYW1lIjoiQ3VzdGVyOCJ9.fiXKdkAhi5tQZH7ZtKEJkdiWiru8rQzktw7wu_G25Ek' \
  --header 'Content-Type: application/json' \
  --data-raw '{
  "username": "Custer8",
  "password": "1234"
}'
```

[代码变动](https://github.com/CusterFun/rust-exercises/commit/86c34e260700dbd359e204528c0a1a4a262751c6#diff-4ac4689c9e9c17852be3d67d135a611413354404d7faf3e1e83799a39fe18525)

## 7. Creating Tasks

### 在 `handler` 中验证 `request`

首先新增 `types/src/task.rs` 文件

```rust
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct RequestTask {
    #[validate(
        required(message = "missing task title"),
        length(min = 1, max = 6, message = "task title length should >1 and <7")
    )]
    pub title: Option<String>,
    pub priority: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseTask {
    pub id: i32,
    pub title: String,
    pub priority: Option<String>,
    pub description: Option<String>,
    pub completed_at: Option<String>,
}

#[derive(Serialize)]
pub struct ResponseDataTask {
    pub data: ResponseTask,
}
```

注意这里需要在 `types` 这个 `crate` 中添加 `validator` 第三方库

```shell
cargo add validator -F derive
```

然后新增 `server/src/api/tasks/create_task.rs` 文件夹和文件

```rust
use axum::{extract::State, http::StatusCode, Extension, Json};
use entity::{tasks, users::Model as UserModel};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TryIntoModel};
use types::task::{RequestTask, ResponseDataTask, ResponseTask};
use validator::Validate;

use crate::util::app_error::AppError;

pub async fn create_task(
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
    Json(request_task): Json<RequestTask>,
) -> Result<(StatusCode, Json<ResponseDataTask>), AppError> {
    // 验证请求的数据
    if let Err(err) = request_task.validate() {
        let field_errors = err.field_errors();
        for (_, error) in field_errors {
            return Err(AppError::new(
                StatusCode::BAD_REQUEST,
                error
                    .first()
                    .unwrap()
                    .clone()
                    .message
                    .unwrap() // .unwrap_or_else(|| "Title shouldn't correct!".into())
                    .to_string(),
            ));
        }
    }
    // 新建待保存数据的对象
    let new_task = tasks::ActiveModel {
        priority: Set(request_task.priority),
        title: Set(request_task.title.unwrap_or_default()),
        description: Set(request_task.description),
        user_id: Set(Some(user.id)),
        ..Default::default()
    };
    // 保存数据到数据库
    let task = new_task
        .save(&db)
        .await
        .map_err(|err| {
            eprintln!("Error creating task: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error creating task")
        })?
        .try_into_model()
        .map_err(|err| {
            eprintln!("Error converting task after creating it: {err:?}");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error converting task")
        })?;
    // 返回 Json 数据
    let response = ResponseDataTask {
        data: ResponseTask {
            id: task.id,
            title: task.title,
            priority: task.priority,
            description: task.description,
            completed_at: task.completed_at.map(|time| time.to_string()),
        },
    };
    Ok((StatusCode::CREATED, Json(response)))
}
```

修改路由 `http://localhost:3000/api/v1/task` 新增 `create_task` 路由处理函数

```rust
use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{
    api::{
        hello::hello,
        tasks::create_task::create_task,
        users::{create_user::create_user, login::login, logout::logout},
    },
    app_state::AppState,
    middleware::require_authentication::require_authentication,
};

pub async fn create_router(app_state: AppState) -> Router {
    let user_routes_auth =
        Router::new()
            .route("/logout", post(logout))
            .route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                require_authentication,
            ));
    let user_routes = Router::new()
        .route("/", post(create_user))
        .route("/login", post(login))
        .merge(user_routes_auth);

    let task_routes =
        Router::new()
            .route("/", post(create_task))
            .route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                require_authentication,
            ));

    let api_routes = Router::new()
        .nest("/users", user_routes)
        .nest("/tasks", task_routes);

    Router::new()
        .route("/", get(hello))
        .nest("/api/v1/", api_routes)
        .with_state(app_state)
}
```

<details><summary>此时发送 curl</summary>

1. 未传入 title
```shell
> curl -X POST \
  'http://localhost:3000/api/v1/tasks' \
  --header 'Accept: */*' \
  --header 'User-Agent: Thunder Client (https://www.thunderclient.com)' \
  --header 'x-token: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2NzYyMTUxNzUsInVzZXJuYW1lIjoiQ3VzdGVyOCJ9.CabRnDVs6KXX701eAYjrlPqT7fYcokTzgniJOozMilo' \
  --header 'Content-Type: application/json' \
  --data-raw '{
  "priority": "A"
}'
```
返回 json 数据

```json
{
  "error": "missing task title"
}
```

2. 传入的 title 长度错误

```shell
curl -X POST \
  'http://localhost:3000/api/v1/tasks' \
  --header 'Accept: */*' \
  --header 'User-Agent: Thunder Client (https://www.thunderclient.com)' \
  --header 'x-token: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2NzYyMTUxNzUsInVzZXJuYW1lIjoiQ3VzdGVyOCJ9.CabRnDVs6KXX701eAYjrlPqT7fYcokTzgniJOozMilo' \
  --header 'Content-Type: application/json' \
  --data-raw '{
  "title": "",
  "priority": "A"
}'
```

返回的 json
```json
{
  "error": "task title length should >1 and <7"
}
```

3. 传入正确的请求

```shell
curl -X POST \
  'http://localhost:3000/api/v1/tasks' \
  --header 'Accept: */*' \
  --header 'User-Agent: Thunder Client (https://www.thunderclient.com)' \
  --header 'x-token: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2NzYyMTUxNzUsInVzZXJuYW1lIjoiQ3VzdGVyOCJ9.CabRnDVs6KXX701eAYjrlPqT7fYcokTzgniJOozMilo' \
  --header 'Content-Type: application/json' \
  --data-raw '{
  "title": "1",
  "priority": "A"
}'
```

返回的 json

```json
{
  "data": {
    "id": 16,
    "title": "1",
    "priority": "A",
    "description": null,
    "completed_at": null
  }
}
```
</details>

[代码变动](https://github.com/CusterFun/rust-exercises/commit/5c3b398e72a75ec40ab2795ff170227dbd46575b#diff-529f4564fd8fa8f2495c6df266d8e913c026c20acbbf38be70fb241b4141332f)

### 在 `request` 提取器中验证，进入 `handler` 函数之前

新建一个 `server/src/api/tasks/create_task_extractor.rs` 自定义提取器，用来从请求中提取数据，进行验证。

自定义提取器[官方文档](https://docs.rs/axum/latest/axum/extract/index.html#defining-custom-extractors)

```rust
use axum::{
    async_trait,
    body::HttpBody,
    extract::FromRequest,
    http::{Request, StatusCode},
    BoxError, Json, RequestExt,
};
use serde::Deserialize;
use validator::Validate;

use crate::util::app_error::AppError;

#[derive(Deserialize, Validate)]
pub struct ValidateCreateTask {
    #[validate(length(min = 1, max = 1))]
    pub priority: Option<String>,
    #[validate(
        required(message = "missing task title"),
        length(min = 1, max = 6, message = "task title length should >1 and <7")
    )]
    pub title: Option<String>,
    pub description: Option<String>,
}

#[async_trait]
impl<S, B> FromRequest<S, B> for ValidateCreateTask
where
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(task) = req
            .extract::<Json<ValidateCreateTask>, _>()
            .await
            .map_err(|err| {
                eprintln!("Error getting bytes in custom create task extractor: {err:?}");
                AppError::new(StatusCode::BAD_REQUEST, "Internal server error")
            })?;

        if let Err(errors) = task.validate() {
            let field_errors = errors.field_errors();
            for (_, error) in field_errors {
                return Err(AppError::new(
                    StatusCode::BAD_REQUEST,
                    error.first().unwrap().clone().message.unwrap().to_string(),
                ));
            }
        }

        Ok(task)
    }
}
```

修改 `create_task.rs` 文件

```rust
use axum::{extract::State, http::StatusCode, Extension, Json};
use entity::{tasks, users::Model as UserModel};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TryIntoModel};
use types::task::{ResponseDataTask, ResponseTask};

use crate::util::app_error::AppError;

use super::create_task_extractor::ValidateCreateTask;

pub async fn create_task(
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
    // Json(request_task): Json<RequestTask>,
    task: ValidateCreateTask,
) -> Result<(StatusCode, Json<ResponseDataTask>), AppError> {
    /*
    if let Err(err) = request_task.validate() {
        let field_errors = err.field_errors();
        for (_, error) in field_errors {
            return Err(AppError::new(
                StatusCode::BAD_REQUEST,
                error
                    .first()
                    .unwrap()
                    .clone()
                    .message
                    .unwrap() // .unwrap_or_else(|| "Title shouldn't correct!".into())
                    .to_string(),
            ));
        }
    }
    let new_task = tasks::ActiveModel {
        priority: Set(request_task.priority),
        title: Set(request_task.title.unwrap_or_default()),
        description: Set(request_task.description),
        user_id: Set(Some(user.id)),
        ..Default::default()
    };
    */
    let new_task = tasks::ActiveModel {
        priority: Set(task.priority),
        title: Set(task.title.unwrap_or_default()),
        description: Set(task.description),
        user_id: Set(Some(user.id)),
        ..Default::default()
    };
    let task = new_task
        .save(&db)
        .await
        .map_err(|err| {
            eprintln!("Error creating task: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error creating task")
        })?
        .try_into_model()
        .map_err(|err| {
            eprintln!("Error converting task after creating it: {err:?}");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error converting task")
        })?;
    let response = ResponseDataTask {
        data: ResponseTask {
            id: task.id,
            title: task.title,
            priority: task.priority,
            description: task.description,
            completed_at: task.completed_at.map(|time| time.to_string()),
        },
    };
    Ok((StatusCode::CREATED, Json(response)))
} 
```

[代码变动](https://github.com/CusterFun/rust-exercises/commit/460b7c4090e5617ebf554fcf7b1addbeb10859a4#diff-529f4564fd8fa8f2495c6df266d8e913c026c20acbbf38be70fb241b4141332f)

## 8. Getting All Tasks

> 注意
> 这里应该只能获取自己的任务
> 退出登录后不能获取任务
> 不能获取别人的任务
> 不能获取已经删除的任务

新建文件 `server/src/api/tasks/get_all_tasks.rs`

```rust
use axum::http::StatusCode;
use axum::{extract::State, Extension, Json};
use entity::users::Model as UserModel;
use entity::{prelude::*, tasks};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use types::task::{ResponseDataTasks, ResponseTask};

use crate::util::app_error::AppError;

pub async fn get_all_tasks(
    Extension(user): Extension<UserModel>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ResponseDataTasks>, AppError> {
    let tasks = Tasks::find()
        .filter(tasks::Column::UserId.eq(Some(user.id)))
        .filter(tasks::Column::DeletedAt.is_null())
        .all(&db)
        .await
        .map_err(|err| {
            eprintln!("Error getting all tasks: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error getting all tasks")
        })?
        .into_iter()
        .map(|task| ResponseTask {
            id: task.id,
            title: task.title,
            priority: task.priority,
            description: task.description,
            completed_at: task.completed_at.map(|time| time.to_string()),
        })
        .collect::<Vec<ResponseTask>>();

    Ok(Json(ResponseDataTasks { data: tasks }))
}
```

> 注意这里 **通过将函数应用于包含的值，将 `Option<T>` 映射到 `Option<U>`。**

```rust
if let Some(time) = task.completed_at { 
    Some(time.to_string()) 
} else { 
    None 
}
```

使用 `map` 将 `Option<T>` 映射到 `Option<U>`

```rust
task.completed_at.map(|time| time.to_string())
```

> 注意这里使用 `map` 将 `Vec<Model>` 转换为 `Vec<ResponseTask>`

```rust
    .into_iter()
        .map(|task| ResponseTask {
            id: task.id,
            title: task.title,
            priority: task.priority,
            description: task.description,
            completed_at: task.completed_at.map(|time| time.to_string()),
        })
        .collect::<Vec<ResponseTask>>();
```

> 这里也可以使用 `sea-orm` 的 `.into_model::<ResponseTask>()` 方法将 `Vec<Model>` 转换为 `Vec<ResponseTask>`，只需要修改对应的 `ResponseTask` 

```rust
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct RequestTask {
    #[validate(
        required(message = "missing task title"),
        length(min = 1, max = 6, message = "task title length should >1 and <7")
    )]
    pub title: Option<String>,
    pub priority: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, FromQueryResult)]
pub struct ResponseTask {
    pub id: i32,
    pub title: String,
    pub priority: Option<String>,
    pub description: Option<String>,
    pub completed_at: Option<String>,
}

#[derive(Serialize)]
pub struct ResponseDataTask {
    pub data: ResponseTask,
}

#[derive(Serialize)]
pub struct ResponseDataTasks {
    pub data: Vec<ResponseTask>,
}
```

在 `get_all_tasks` 中使用 

```rust
use axum::http::StatusCode;
use axum::{extract::State, Extension, Json};
use entity::users::Model as UserModel;
use entity::{prelude::*, tasks};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use types::task::{ResponseDataTasks, ResponseTask};

use crate::util::app_error::AppError;

pub async fn get_all_tasks(
    Extension(user): Extension<UserModel>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ResponseDataTasks>, AppError> {
    let tasks = Tasks::find()
        .filter(tasks::Column::UserId.eq(Some(user.id)))
        .filter(tasks::Column::DeletedAt.is_null())
        .into_model::<ResponseTask>()
        .all(&db)
        .await
        .map_err(|err| {
            eprintln!("Error getting all tasks: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error getting all tasks")
        })?;

    Ok(Json(ResponseDataTasks { data: tasks }))
}
```

[代码变动](https://github.com/CusterFun/rust-exercises/commit/16ae700f8b36f880c01d3abd237b745bfe6c538d#diff-f2c270ad790a4347fd28c08f637dd17c5facd639b45b82c4e7680d0cefa1814d)

### 创建新用户时添加默认task
修改 `server/src/api/users/create_user.rs`

```rust
use axum::{extract::State, http::StatusCode, Json};
use entity::tasks::Entity as Tasks;
use entity::{tasks, users};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, TryIntoModel,
};
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
        .map_err(|err| {
            let error_message = err.to_string();
            if error_message
                .contains("duplicate key value violates unique constraint \"users_username_key\"")
            {
                AppError::new(
                    StatusCode::BAD_REQUEST,
                    "Username already taken, try again with a different user name",
                )
            } else {
                eprintln!("Error creating user: {:?}", &err);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
            }
        })?
        .try_into_model()
        .map_err(|err| {
            eprintln!("Error converting user back into model: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
        })?;

    let default_tasks = Tasks::find()
        .filter(tasks::Column::IsDefault.eq(Some(true)))
        .filter(tasks::Column::DeletedAt.is_null())
        .all(&db)
        .await
        .map_err(|err| {
            eprintln!("Error getting default tasks: {:?}", err);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error applying default tasks to new account",
            )
        })?;
    for default_task in default_tasks {
        let task = tasks::ActiveModel {
            priority: Set(default_task.priority),
            title: Set(default_task.title),
            completed_at: Set(default_task.completed_at),
            description: Set(default_task.description),
            deleted_at: Set(default_task.deleted_at),
            user_id: Set(Some(user.id)),
            ..Default::default()
        };

        task.save(&db).await.map_err(|err| {
            eprintln!("Error creating task from default: {:?}", err);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error saving new default task for user",
            )
        })?;
    }
    Ok(Json(ResponseDataUser {
        data: ResponseUser {
            id: user.id,
            username: user.username,
            token: user.token.unwrap_or_default(),
        },
    }))
}
```

[代码变动](https://github.com/CusterFun/rust-exercises/commit/2c4c0d769ed06ccd528d7502d3ff34122ee58c85#diff-ee6af3e44180d400670d53d2b574ab6e4526b5a568a6d40c244b0db1b7f4f0bb)

### 重构代码

```rust
use axum::{extract::State, http::StatusCode, Json};
use entity::tasks::Entity as Tasks;
use entity::{tasks, users};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, TryIntoModel,
};
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
        .map_err(|err| {
            let error_message = err.to_string();
            if error_message
                .contains("duplicate key value violates unique constraint \"users_username_key\"")
            {
                AppError::new(
                    StatusCode::BAD_REQUEST,
                    "Username already taken, try again with a different user name",
                )
            } else {
                eprintln!("Error creating user: {:?}", &err);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
            }
        })?
        .try_into_model()
        .map_err(|err| {
            eprintln!("Error converting user back into model: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
        })?;

    create_default_tasks_for_user(&db, &user).await?;

    Ok(Json(ResponseDataUser {
        data: ResponseUser {
            id: user.id,
            username: user.username,
            token: user.token.unwrap_or_default(),
        },
    }))
}

async fn create_default_tasks_for_user(
    db: &DatabaseConnection,
    user: &users::Model,
) -> Result<(), AppError> {
    let default_tasks = Tasks::find()
        .filter(tasks::Column::IsDefault.eq(Some(true)))
        .filter(tasks::Column::DeletedAt.is_null())
        .all(db)
        .await
        .map_err(|err| {
            eprintln!("Error getting default tasks: {:?}", err);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error applying default tasks to new account",
            )
        })?;
    Ok(for default_task in default_tasks {
        let task = tasks::ActiveModel {
            priority: Set(default_task.priority),
            title: Set(default_task.title),
            completed_at: Set(default_task.completed_at),
            description: Set(default_task.description),
            deleted_at: Set(default_task.deleted_at),
            user_id: Set(Some(user.id)),
            ..Default::default()
        };

        task.save(db).await.map_err(|err| {
            eprintln!("Error creating task from default: {:?}", err);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error saving new default task for user",
            )
        })?;
    })
}
```

[代码变动](https://github.com/CusterFun/rust-exercises/commit/f1caf9057b95de308b13c5395ad54c1be0f385ea#diff-ee6af3e44180d400670d53d2b574ab6e4526b5a568a6d40c244b0db1b7f4f0bb)

## 9. Get One Task
> should be able to get my task
> should not be able to get task when logged out
> should not be able to get another users task

新建文件 `server/src/api/tasks/get_one_task.rs`

```rust
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use entity::users::Model as UserModel;
use entity::{prelude::*, tasks};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use types::task::{ResponseDataTask, ResponseTask};

use crate::util::app_error::AppError;

pub async fn get_one_task(
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
) -> Result<Json<ResponseDataTask>, AppError> {
    let task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::UserId.eq(Some(user.id)))
        .filter(tasks::Column::DeletedAt.is_null())
        // .into_model::<ResponseTask>()
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error when getting task: {err:?}");
            AppError::new(
                StatusCode::BAD_REQUEST,
                "We got an error when attempting to load your task",
            )
        })?;

    let response = if let Some(task) = task {
        // task
        ResponseTask {
            id: task.id,
            title: task.title,
            priority: task.priority,
            description: task.description,
            completed_at: task.completed_at.map(|time| time.to_string()),
        }
    } else {
        return Err(AppError::new(StatusCode::NOT_FOUND, "not found"));
    };

    Ok(Json(ResponseDataTask { data: response }))
}
```

[代码变动](https://github.com/CusterFun/rust-exercises/commit/07ab241688b0a808109f8bf64b924addf0a19935#diff-f37ba02f6174cc71746f0285aa5689cad496436d40f9cc4dfbb63f4f1481b508)

## 10. Update Tasks

> should be able to mark a task as completed
> should be able to mark a task as not completed
> should be able to update all fields in the task
> should can_update_some_of_the_task_without_losing_data 可以在不丢失数据的情况下更新一些任务
> should can uncomplete a task with an update 可以通过更新取消完成任务
> should not be able to mark other users tasks as completed
> should not be able to mark other users tasks as not completed
> should not be able to update other users tasks

### should be able to mark a task as completed
新建文件 `server/src/api/tasks/update_tasks.rs`

```rust
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension,
};
use chrono::Utc;
use entity::tasks::{self, Entity as Tasks};
use entity::users::Model as UserModel;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};

use crate::util::app_error::AppError;

pub async fn mark_completed(
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
) -> Result<(), AppError> {
    let task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::UserId.eq(Some(user.id)))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error getting task to update: {err:?}");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "An error happend")
        })?;
    let mut task = if let Some(task) = task {
        task.into_active_model()
    } else {
        return Err(AppError::new(StatusCode::NOT_FOUND, "Task not found"));
    };

    let now = Utc::now();
    task.completed_at = Set(Some(now.into()));
    task.save(&db).await.map_err(|err| {
        eprintln!("Error marking task as completed: {err:?}");
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error while updating completed at",
        )
    })?;
    Ok(())
}
```

测试 curl 

```shell
curl -X PUT \
  'http://localhost:3000/api/v1/tasks/21/completed' \
  --header 'Accept: */*' \
  --header 'User-Agent: Thunder Client (https://www.thunderclient.com)' \
  --header 'x-token: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2Nzc5MTgyMTgsInVzZXJuYW1lIjoiQ3VzdGVyMTEifQ.-yjCTEbb-3Rqd5I4jgP_6T3-h_oHMm1yyQEbkZyKM1U'
```

返回200，查看该任务是否已经标记为完成

```shell
curl -X GET \
  'http://localhost:3000/api/v1/tasks/21' \
  --header 'Accept: */*' \
  --header 'User-Agent: Thunder Client (https://www.thunderclient.com)' \
  --header 'x-token: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2Nzc5MTgyMTgsInVzZXJuYW1lIjoiQ3VzdGVyMTEifQ.-yjCTEbb-3Rqd5I4jgP_6T3-h_oHMm1yyQEbkZyKM1U'
```

返回 JSON

```json
{
  "data": {
    "id": 21,
    "title": "See my details for by clicking me",
    "priority": "B",
    "description": "My description can be changed",
    "completed_at": "2023-03-04 07:24:26.907190 +00:00"
  }
}
```

[代码变动](https://github.com/CusterFun/rust-exercises/commit/37a89c31b556b096833b2acf87beea57f37cdca9#diff-b7136d3e53a5279956cad1bc3652f43b2e12d7d0e3a83669358d0038b10a2a2b)

### should be able to mark a task as not completed

修改文件 `server/src/api/tasks/update_tasks.rs` 增加 `mark_uncompleted` 函数

```rust

pub async fn mark_uncompleted(
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
) -> Result<(), AppError> {
    let task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::UserId.eq(Some(user.id)))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error getting task to update: {err:?}");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "An error happend")
        })?;
    let mut task = if let Some(task) = task {
        task.into_active_model()
    } else {
        return Err(AppError::new(StatusCode::NOT_FOUND, "Task not found"));
    };

    task.completed_at = Set(None);
    task.save(&db).await.map_err(|err| {
        eprintln!("Error marking task as uncompleted: {err:?}");
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error while updating uncompleted at",
        )
    })?;
    Ok(())
}
```

测试 curl
```shell
curl -X PUT \
  'http://localhost:3000/api/v1/tasks/21/uncompleted' \
  --header 'Accept: */*' \
  --header 'User-Agent: Thunder Client (https://www.thunderclient.com)' \
  --header 'x-token: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2Nzc5MTgyMTgsInVzZXJuYW1lIjoiQ3VzdGVyMTEifQ.-yjCTEbb-3Rqd5I4jgP_6T3-h_oHMm1yyQEbkZyKM1U'
```

查看该任务
```shell
curl -X GET \
  'http://localhost:3000/api/v1/tasks/21' \
  --header 'Accept: */*' \
  --header 'User-Agent: Thunder Client (https://www.thunderclient.com)' \
  --header 'x-token: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2Nzc5MTgyMTgsInVzZXJuYW1lIjoiQ3VzdGVyMTEifQ.-yjCTEbb-3Rqd5I4jgP_6T3-h_oHMm1yyQEbkZyKM1U'
```

返回结果为 

```json
{
  "data": {
    "id": 21,
    "title": "See my details for by clicking me",
    "priority": "B",
    "description": "My description can be changed",
    "completed_at": null
  }
}
```

[代码变动](https://github.com/CusterFun/rust-exercises/commit/bd6d58288527d1ad69a8e8536d9ffdf0c2edb851#diff-b7136d3e53a5279956cad1bc3652f43b2e12d7d0e3a83669358d0038b10a2a2b)

### should be able to update all fields in the task

在 `types` 中 新增 `chrono` 第三方库

```shell
cargo add chrono -F serde
cargo add serde_with
```

如果为空就不更新该字段，就需要使用 [serde_with](https://crates.io/crates/serde_with) 的 [double option](https://docs.rs/serde_with/latest/serde_with/rust/double_option/index.html)

> 区分更新字段时：忽略更新该字段、更新该字段为NULL、更新该字段为新值

- None: Represents a missing value.
- Some(None): Represents a null value.
- Some(Some(value)): Represents an existing value

```rust
#[derive(Deserialize, Serialize)]
struct Doc {
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    a: Option<Option<u8>>,
}
// Missing Value
let s = r#"{}"#;
assert_eq!(Doc { a: None }, serde_json::from_str(s).unwrap());
assert_eq!(s, serde_json::to_string(&Doc { a: None }).unwrap());

// Unset Value
let s = r#"{"a":null}"#;
assert_eq!(Doc { a: Some(None) }, serde_json::from_str(s).unwrap());
assert_eq!(s, serde_json::to_string(&Doc { a: Some(None) }).unwrap());

// Existing Value
let s = r#"{"a":5}"#;
assert_eq!(Doc { a: Some(Some(5)) }, serde_json::from_str(s).unwrap());
assert_eq!(s, serde_json::to_string(&Doc { a: Some(Some(5)) }).unwrap());
```

接着修改请求的数据结构 `types/src/task.rs`

<details>

```rust
use chrono::{DateTime, FixedOffset};
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct RequestTask {
    #[validate(
        required(message = "missing task title"),
        length(min = 1, max = 6, message = "task title length should >1 and <7")
    )]
    pub title: Option<String>,
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub priority: Option<Option<String>>,
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub description: Option<Option<String>>,
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub completed_at: Option<Option<DateTime<FixedOffset>>>,
}

#[derive(Serialize, Deserialize, FromQueryResult)]
pub struct ResponseTask {
    pub id: i32,
    pub title: String,
    pub priority: Option<String>,
    pub description: Option<String>,
    pub completed_at: Option<String>,
}

#[derive(Serialize)]
pub struct ResponseDataTask {
    pub data: ResponseTask,
}

#[derive(Serialize)]
pub struct ResponseDataTasks {
    pub data: Vec<ResponseTask>,
}
```

</details>
修改文件 `server/src/api/tasks/update_tasks.rs` 增加 `update_task` 函数

```rust

pub async fn update_task(
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
    Json(request_task): Json<RequestTask>,
) -> Result<(), AppError> {
    let task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::UserId.eq(Some(user.id)))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error getting task to update: {err:?}");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "An error happend")
        })?;
    let mut task = if let Some(task) = task {
        task.into_active_model()
    } else {
        return Err(AppError::new(StatusCode::NOT_FOUND, "Task not found"));
    };

    if let Some(priority) = request_task.priority {
        task.priority = Set(priority);
    }
    if let Some(description) = request_task.description {
        task.description = Set(description);
    }
    if let Some(title) = request_task.title {
        task.title = Set(title);
    }
    if let Some(completed_at) = request_task.completed_at {
        task.completed_at = Set(completed_at);
    }

    task.save(&db).await.map_err(|err| {
        eprintln!("Error marking task as uncompleted: {err:?}");
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error while updating uncompleted at",
        )
    })?;
    Ok(())
}
```

进行 curl 测试，缺省的字段为不更新，字段设置为null的更新为null，设置为新值的更新为新值

```shell
curl -X PATCH \
  'http://localhost:3000/api/v1/tasks/21' \
  --header 'Accept: */*' \
  --header 'User-Agent: Thunder Client (https://www.thunderclient.com)' \
  --header 'x-token: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2Nzc5MjIwNTIsInVzZXJuYW1lIjoiQ3VzdGVyMTEifQ.HKMhEbxvL4yDmICUiEH7YpkSmrqn1ODgjUhC8KHNt-o' \
  --header 'Content-Type: application/json' \
  --data-raw '{
  "title": "123",
  "priority": "A",
  "description": "desc",
  "completed_at": null
}'
```

查看更新之后的 JSON

```shell
curl -X GET \
  'http://localhost:3000/api/v1/tasks/21' \
  --header 'Accept: */*' \
  --header 'User-Agent: Thunder Client (https://www.thunderclient.com)' \
  --header 'x-token: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2Nzc5MjIwNTIsInVzZXJuYW1lIjoiQ3VzdGVyMTEifQ.HKMhEbxvL4yDmICUiEH7YpkSmrqn1ODgjUhC8KHNt-o'
```

```json
{
  "data": {
    "id": 21,
    "title": "123",
    "priority": "A",
    "description": "desc",
    "completed_at": null
  }
}
```

更新的 `router.rs` 代码

<details>

```rust
use axum::{
    middleware,
    routing::{get, patch, post, put},
    Router,
};

use crate::{
    api::{
        hello::hello,
        tasks::{
            create_task::create_task,
            get_all_tasks::get_all_tasks,
            get_one_task::get_one_task,
            update_tasks::{mark_completed, mark_uncompleted, update_task},
        },
        users::{create_user::create_user, login::login, logout::logout},
    },
    app_state::AppState,
    middleware::require_authentication::require_authentication,
};

pub async fn create_router(app_state: AppState) -> Router {
    let user_routes_auth =
        Router::new()
            .route("/logout", post(logout))
            .route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                require_authentication,
            ));
    let user_routes = Router::new()
        .route("/", post(create_user))
        .route("/login", post(login))
        .merge(user_routes_auth);

    let task_routes = Router::new()
        .route("/", post(create_task))
        .route("/", get(get_all_tasks))
        .route("/:task_id", get(get_one_task))
        .route("/:task_id/completed", put(mark_completed))
        .route("/:task_id/uncompleted", put(mark_uncompleted))
        .route("/:task_id", patch(update_task))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            require_authentication,
        ));

    let api_routes = Router::new()
        .nest("/users", user_routes)
        .nest("/tasks", task_routes);

    Router::new()
        .route("/", get(hello))
        .nest("/api/v1/", api_routes)
        .with_state(app_state)
}
```
</details>

[代码更新](https://github.com/CusterFun/rust-exercises/commit/5300c691c774affa0f548dc2d48816ba8dc8de41#diff-b7136d3e53a5279956cad1bc3652f43b2e12d7d0e3a83669358d0038b10a2a2b)

## 11. Soft Deleting Tasks

新建文件 `src/api/tasks/delete_task.rs`

```rust
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension,
};
use entity::{prelude::*, tasks, users::Model as UserModel};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};

use crate::util::app_error::AppError;

pub async fn soft_delete_task(
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
) -> Result<(), AppError> {
    let task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::UserId.eq(Some(user.id)))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error deleteing task: {err:?}");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error deleting the task")
        })?;

    let mut task = if let Some(task) = task {
        task.into_active_model()
    } else {
        return Err(AppError::new(StatusCode::NOT_FOUND, "Task not found"));
    };

    let now = chrono::Utc::now();

    task.deleted_at = Set(Some(now.into()));

    task.save(&db).await.map_err(|err| {
        eprintln!("Error deleting task: {err:?}");
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error while deleting task",
        )
    })?;

    Ok(())
}
```

使用 curl 测试

```shell
curl -X DELETE \
  'http://localhost:3000/api/v1/tasks/21' \
  --header 'Accept: */*' \
  --header 'User-Agent: Thunder Client (https://www.thunderclient.com)' \
  --header 'x-token: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2Nzc5MjIwNTIsInVzZXJuYW1lIjoiQ3VzdGVyMTEifQ.HKMhEbxvL4yDmICUiEH7YpkSmrqn1ODgjUhC8KHNt-o'
```

查看是否删除该任务

```shell
curl -X GET \
  'http://localhost:3000/api/v1/tasks/21' \
  --header 'Accept: */*' \
  --header 'User-Agent: Thunder Client (https://www.thunderclient.com)' \
  --header 'x-token: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2Nzc5MjIwNTIsInVzZXJuYW1lIjoiQ3VzdGVyMTEifQ.HKMhEbxvL4yDmICUiEH7YpkSmrqn1ODgjUhC8KHNt-o'
```

返回

```json
{
  "error": "not found"
}
```

[代码变动](https://github.com/CusterFun/rust-exercises/commit/af4e92b03d7fd1f9bdf6a33cb3fe80d6aa328dbc#diff-7687c4192b778c8847619360ad1d0167537616c606c5073d428bed846d3fbd26)

## 12. Refactoring Queries
### 简化 `create_task.rs`
新增 `server/src/queries/mod.rs` 文件夹和文件 `task_queries.rs` 和文件`user_queries.rs`, 并修改 `mod.rs` 文件

```rust
pub mod task_queries;
pub mod user_queries;
```

首先抽取 `src/api/tasks/create_task.rs` 中的代码

```rust
use axum::{extract::State, http::StatusCode, Extension, Json};
use entity::{tasks, users::Model as UserModel};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TryIntoModel};
use types::task::{ResponseDataTask, ResponseTask};

use crate::util::app_error::AppError;

use super::create_task_extractor::ValidateCreateTask;

pub async fn create_task(
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
    // Json(request_task): Json<RequestTask>,
    task: ValidateCreateTask,
) -> Result<(StatusCode, Json<ResponseDataTask>), AppError> {
    /*
    if let Err(err) = request_task.validate() {
        let field_errors = err.field_errors();
        for (_, error) in field_errors {
            return Err(AppError::new(
                StatusCode::BAD_REQUEST,
                error
                    .first()
                    .unwrap()
                    .clone()
                    .message
                    .unwrap() // .unwrap_or_else(|| "Title shouldn't correct!".into())
                    .to_string(),
            ));
        }
    }
    let new_task = tasks::ActiveModel {
        priority: Set(request_task.priority),
        title: Set(request_task.title.unwrap_or_default()),
        description: Set(request_task.description),
        user_id: Set(Some(user.id)),
        ..Default::default()
    };
    */
    let new_task = tasks::ActiveModel {
        priority: Set(task.priority),
        title: Set(task.title.unwrap_or_default()),
        description: Set(task.description),
        user_id: Set(Some(user.id)),
        ..Default::default()
    };
    let task = new_task
        .save(&db)
        .await
        .map_err(|err| {
            eprintln!("Error creating task: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error creating task")
        })?
        .try_into_model()
        .map_err(|err| {
            eprintln!("Error converting task after creating it: {err:?}");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error converting task")
        })?;
    let response = ResponseDataTask {
        data: ResponseTask {
            id: task.id,
            title: task.title,
            priority: task.priority,
            description: task.description,
            completed_at: task.completed_at.map(|time| time.to_string()),
        },
    };
    Ok((StatusCode::CREATED, Json(response)))
}
```

抽取到 `server/src/queries/task_queries.rs`

```rust
use axum::http::StatusCode;
use entity::{tasks, tasks::Model as TasksModel, users::Model as UsersModel};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TryIntoModel};

use crate::{api::tasks::create_task_extractor::ValidateCreateTask, util::app_error::AppError};

pub async fn create_task(
    task: ValidateCreateTask,
    user: &UsersModel,
    db: &DatabaseConnection,
) -> Result<TasksModel, AppError> {
    let new_task = tasks::ActiveModel {
        priority: Set(task.priority),
        title: Set(task.title.unwrap_or_default()),
        description: Set(task.description),
        user_id: Set(Some(user.id)),
        ..Default::default()
    };

    let task = new_task.save(db).await.map_err(|err| {
        eprintln!("Error creating task: {:?}", err);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error creating task")
    })?;

    convert_active_to_model(task)
}

fn convert_active_to_model(active_task: tasks::ActiveModel) -> Result<TasksModel, AppError> {
    active_task.try_into_model().map_err(|err| {
        eprintln!("Error converting task active model to model: {err:?}");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
    })
}
```

简化 `server/src/api/tasks/create_task.rs` 代码为

```rust
use axum::{extract::State, http::StatusCode, Extension, Json};
use entity::users::Model as UserModel;
use sea_orm::DatabaseConnection;
use types::task::{ResponseDataTask, ResponseTask};

use crate::{queries::task_queries, util::app_error::AppError};

use super::create_task_extractor::ValidateCreateTask;

pub async fn create_task(
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
    task: ValidateCreateTask,
) -> Result<(StatusCode, Json<ResponseDataTask>), AppError> {
    let task = task_queries::create_task(task, &user, &db).await?;
    let response = ResponseDataTask {
        data: ResponseTask {
            id: task.id,
            title: task.title,
            priority: task.priority,
            description: task.description,
            completed_at: task.completed_at.map(|time| time.to_string()),
        },
    };
    Ok((StatusCode::CREATED, Json(response)))
}
```
[代码变动](https://github.com/CusterFun/rust-exercises/commit/63823997e2023f1a57f88e6d6878849d52f20813#diff-529f4564fd8fa8f2495c6df266d8e913c026c20acbbf38be70fb241b4141332f)
### 简化 `delete_task.rs`

在 `src/queries/task_queries.rs` 中新增 `find_task_by_id` 函数

```rust
pub async fn find_task_by_id(
    db: &DatabaseConnection,
    id: i32,
    user_id: i32,
) -> Result<TasksModel, AppError> {
    let task = Tasks::find_by_id(id)
        .filter(tasks::Column::UserId.eq(Some(user_id)))
        .one(db)
        .await
        .map_err(|err| {
            eprintln!("Error getting task by id: {err:?}");
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was an error getting your task",
            )
        })?;

    task.ok_or_else(|| {
        eprintln!("CCould not find task by id: {id:?}");
        AppError::new(StatusCode::NOT_FOUND, "Task not found")
    })
}
```

将原文件 `src/api/tasks/delete_task.rs` 

```rust
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension,
};
use entity::{prelude::*, tasks, users::Model as UserModel};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};

use crate::util::app_error::AppError;

pub async fn soft_delete_task(
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
) -> Result<(), AppError> {
    let task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::UserId.eq(Some(user.id)))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error deleteing task: {err:?}");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error deleting the task")
        })?;

    let mut task = if let Some(task) = task {
        task.into_active_model()
    } else {
        return Err(AppError::new(StatusCode::NOT_FOUND, "Task not found"));
    };

    let now = chrono::Utc::now();

    task.deleted_at = Set(Some(now.into()));

    task.save(&db).await.map_err(|err| {
        eprintln!("Error deleting task: {err:?}");
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error while deleting task",
        )
    })?;

    Ok(())
}
```

简化为 

```rust
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension,
};
use entity::{prelude::*, users::Model as UserModel};
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set,
};

use crate::{queries::task_queries, util::app_error::AppError};

pub async fn soft_delete_task(
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
) -> Result<(), AppError> {
    let mut task = task_queries::find_task_by_id(&db, task_id, user.id)
        .await?
        .into_active_model();

    let now = chrono::Utc::now();

    task.deleted_at = Set(Some(now.into()));

    task.save(&db).await.map_err(|err| {
        eprintln!("Error deleting task: {err:?}");
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error while deleting task",
        )
    })?;

    Ok(())
}
```

### 简化 `save_active_task`

在 `src/queries/task_queries.rs` 中新增 `save_active_task` 函数

```rust
use axum::http::StatusCode;
use entity::{prelude::*, tasks, tasks::Model as TasksModel, users::Model as UsersModel};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, TryIntoModel,
};

use crate::{api::tasks::create_task_extractor::ValidateCreateTask, util::app_error::AppError};

pub async fn create_task(
    task: ValidateCreateTask,
    user: &UsersModel,
    db: &DatabaseConnection,
) -> Result<TasksModel, AppError> {
    let new_task = tasks::ActiveModel {
        priority: Set(task.priority),
        title: Set(task.title.unwrap_or_default()),
        description: Set(task.description),
        user_id: Set(Some(user.id)),
        ..Default::default()
    };

    save_active_task(&db, new_task).await
}

pub async fn find_task_by_id(
    db: &DatabaseConnection,
    id: i32,
    user_id: i32,
) -> Result<TasksModel, AppError> {
    let task = Tasks::find_by_id(id)
        .filter(tasks::Column::UserId.eq(Some(user_id)))
        .one(db)
        .await
        .map_err(|err| {
            eprintln!("Error getting task by id: {err:?}");
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was an error getting your task",
            )
        })?;

    task.ok_or_else(|| {
        eprintln!("CCould not find task by id: {id:?}");
        AppError::new(StatusCode::NOT_FOUND, "Task not found")
    })
}

pub async fn save_active_task(
    db: &DatabaseConnection,
    task: tasks::ActiveModel,
) -> Result<TasksModel, AppError> {
    let task = task.save(db).await.map_err(|err| {
        eprintln!("Error saving task: {err:?}");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error saving task")
    })?;
    convert_active_to_model(task)
}

fn convert_active_to_model(active_task: tasks::ActiveModel) -> Result<TasksModel, AppError> {
    active_task.try_into_model().map_err(|err| {
        eprintln!("Error converting task active model to model: {err:?}");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
    })
}
```

继续简化 `src/api/tasks/delete_task.rs` 文件

```rust
use axum::{
    extract::{Path, State},
    Extension,
};
use entity::users::Model as UserModel;
use sea_orm::{DatabaseConnection, IntoActiveModel, Set};

use crate::{
    queries::task_queries::{find_task_by_id, save_active_task},
    util::app_error::AppError,
};

pub async fn soft_delete_task(
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
) -> Result<(), AppError> {
    let mut task = find_task_by_id(&db, task_id, user.id)
        .await?
        .into_active_model();

    let now = chrono::Utc::now();

    task.deleted_at = Set(Some(now.into()));

    save_active_task(&db, task).await?;

    Ok(())
}
```
[代码变动](https://github.com/CusterFun/rust-exercises/commit/85bac0d418c068e96fcfc712525253d6239230bc#diff-7687c4192b778c8847619360ad1d0167537616c606c5073d428bed846d3fbd26)
### 简化 `get_all_task.rs`

在 `src/queries/task_queries.rs` 中新增 `find_all_tasks` 函数

```rust

pub async fn get_all_tasks(
    db: &DatabaseConnection,
    user_id: i32,
    delete: bool,
) -> Result<Vec<TasksModel>, AppError> {
    let mut query = Tasks::find().filter(tasks::Column::UserId.eq(Some(user_id)));
    if delete {
        query = query.filter(tasks::Column::DeletedAt.is_null());
    }

    query
        // .into_model::<ResponseTask>()
        .all(db)
        .await
        .map_err(|err| {
            eprintln!("Error getting all tasks: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error getting all tasks")
        })
}
```

将原文件 `src/api/tasks/get_all_tasks.rs`

```rust
use axum::http::StatusCode;
use axum::{extract::State, Extension, Json};
use entity::users::Model as UserModel;
use entity::{prelude::*, tasks};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use types::task::{ResponseDataTasks, ResponseTask};

use crate::util::app_error::AppError;

pub async fn get_all_tasks(
    Extension(user): Extension<UserModel>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ResponseDataTasks>, AppError> {
    let tasks = Tasks::find()
        .filter(tasks::Column::UserId.eq(Some(user.id)))
        .filter(tasks::Column::DeletedAt.is_null())
        // .into_model::<ResponseTask>()
        .all(&db)
        .await
        .map_err(|err| {
            eprintln!("Error getting all tasks: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error getting all tasks")
        })?
        .into_iter()
        .map(|task| ResponseTask {
            id: task.id,
            title: task.title,
            priority: task.priority,
            description: task.description,
            completed_at: task.completed_at.map(|time| time.to_string()),
        })
        .collect::<Vec<ResponseTask>>();

    Ok(Json(ResponseDataTasks { data: tasks }))
}
```

简化为 

```rust
use axum::{extract::State, Extension, Json};
use entity::users::Model as UserModel;
use sea_orm::DatabaseConnection;
use types::task::{ResponseDataTasks, ResponseTask};

use crate::queries::task_queries;
use crate::util::app_error::AppError;

pub async fn get_all_tasks(
    Extension(user): Extension<UserModel>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ResponseDataTasks>, AppError> {
    let tasks = task_queries::get_all_tasks(&db, user.id, true)
        .await?
        .into_iter()
        .map(|task| ResponseTask {
            id: task.id,
            title: task.title,
            priority: task.priority,
            description: task.description,
            completed_at: task.completed_at.map(|time| time.to_string()),
        })
        .collect::<Vec<ResponseTask>>();

    Ok(Json(ResponseDataTasks { data: tasks }))
}
```

[代码变动](https://github.com/CusterFun/rust-exercises/commit/59d1c8d8eb611b6f5204b20a2cbef628cf445637#diff-f2c270ad790a4347fd28c08f637dd17c5facd639b45b82c4e7680d0cefa1814d)

### 简化 `get_one_task.rs`

将原文件 `src/api/tasks/get_one_task.rs`

```rust
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use entity::users::Model as UserModel;
use entity::{prelude::*, tasks};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use types::task::{ResponseDataTask, ResponseTask};

use crate::util::app_error::AppError;

pub async fn get_one_task(
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
) -> Result<Json<ResponseDataTask>, AppError> {
    let task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::UserId.eq(Some(user.id)))
        .filter(tasks::Column::DeletedAt.is_null())
        // .into_model::<ResponseTask>()
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error when getting task: {err:?}");
            AppError::new(
                StatusCode::BAD_REQUEST,
                "We got an error when attempting to load your task",
            )
        })?;

    let response = if let Some(task) = task {
        // task
        ResponseTask {
            id: task.id,
            title: task.title,
            priority: task.priority,
            description: task.description,
            completed_at: task.completed_at.map(|time| time.to_string()),
        }
    } else {
        return Err(AppError::new(StatusCode::NOT_FOUND, "not found"));
    };

    Ok(Json(ResponseDataTask { data: response }))
}
```

简化为 

```rust
use axum::{
    extract::{Path, State},
    Extension, Json,
};
use entity::users::Model as UserModel;

use sea_orm::DatabaseConnection;
use types::task::{ResponseDataTask, ResponseTask};

use crate::{queries::task_queries::find_task_by_id, util::app_error::AppError};

pub async fn get_one_task(
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
) -> Result<Json<ResponseDataTask>, AppError> {
    let task = find_task_by_id(&db, task_id, user.id).await?;

    let response = ResponseTask {
        id: task.id,
        title: task.title,
        priority: task.priority,
        description: task.description,
        completed_at: task.completed_at.map(|time| time.to_string()),
    };

    Ok(Json(ResponseDataTask { data: response }))
}
```

[代码变动](https://github.com/CusterFun/rust-exercises/commit/83c68779281a69cc4259234c8f6750bc3c813948#diff-f37ba02f6174cc71746f0285aa5689cad496436d40f9cc4dfbb63f4f1481b508)
### 简化 `update_tasks.rs`

将原文件 `src/api/tasks/update_tasks.rs`

```rust
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use chrono::Utc;
use entity::tasks::{self, Entity as Tasks};
use entity::users::Model as UserModel;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};
use types::task::RequestTask;

use crate::util::app_error::AppError;

pub async fn mark_completed(
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
) -> Result<(), AppError> {
    let task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::UserId.eq(Some(user.id)))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error getting task to update: {err:?}");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "An error happend")
        })?;
    let mut task = if let Some(task) = task {
        task.into_active_model()
    } else {
        return Err(AppError::new(StatusCode::NOT_FOUND, "Task not found"));
    };

    let now = Utc::now();
    task.completed_at = Set(Some(now.into()));
    task.save(&db).await.map_err(|err| {
        eprintln!("Error marking task as completed: {err:?}");
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error while updating completed at",
        )
    })?;
    Ok(())
}

pub async fn mark_uncompleted(
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
) -> Result<(), AppError> {
    let task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::UserId.eq(Some(user.id)))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error getting task to update: {err:?}");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "An error happend")
        })?;
    let mut task = if let Some(task) = task {
        task.into_active_model()
    } else {
        return Err(AppError::new(StatusCode::NOT_FOUND, "Task not found"));
    };

    task.completed_at = Set(None);
    task.save(&db).await.map_err(|err| {
        eprintln!("Error marking task as uncompleted: {err:?}");
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error while updating uncompleted at",
        )
    })?;
    Ok(())
}

pub async fn update_task(
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
    Json(request_task): Json<RequestTask>,
) -> Result<(), AppError> {
    let task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::UserId.eq(Some(user.id)))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error getting task to update: {err:?}");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "An error happend")
        })?;
    let mut task = if let Some(task) = task {
        task.into_active_model()
    } else {
        return Err(AppError::new(StatusCode::NOT_FOUND, "Task not found"));
    };

    if let Some(priority) = request_task.priority {
        task.priority = Set(priority);
    }
    if let Some(description) = request_task.description {
        task.description = Set(description);
    }
    if let Some(title) = request_task.title {
        task.title = Set(title);
    }
    if let Some(completed_at) = request_task.completed_at {
        task.completed_at = Set(completed_at);
    }

    task.save(&db).await.map_err(|err| {
        eprintln!("Error marking task as uncompleted: {err:?}");
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error while updating uncompleted at",
        )
    })?;
    Ok(())
}
```

简化为 

```rust
use axum::{
    extract::{Path, State},
    Extension, Json,
};
use chrono::Utc;
use entity::users::Model as UserModel;
use sea_orm::{DatabaseConnection, IntoActiveModel, Set};
use types::task::RequestTask;

use crate::{
    queries::task_queries::{find_task_by_id, save_active_task},
    util::app_error::AppError,
};

pub async fn mark_completed(
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
) -> Result<(), AppError> {
    let mut task = find_task_by_id(&db, task_id, user.id)
        .await?
        .into_active_model();

    let now = Utc::now();
    task.completed_at = Set(Some(now.into()));

    save_active_task(&db, task).await?;

    Ok(())
}

pub async fn mark_uncompleted(
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
) -> Result<(), AppError> {
    let mut task = find_task_by_id(&db, task_id, user.id)
        .await?
        .into_active_model();

    task.completed_at = Set(None);

    save_active_task(&db, task).await?;

    Ok(())
}

pub async fn update_task(
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
    Json(request_task): Json<RequestTask>,
) -> Result<(), AppError> {
    let mut task = find_task_by_id(&db, task_id, user.id)
        .await?
        .into_active_model();

    if let Some(priority) = request_task.priority {
        task.priority = Set(priority);
    }
    if let Some(description) = request_task.description {
        task.description = Set(description);
    }
    if let Some(title) = request_task.title {
        task.title = Set(title);
    }
    if let Some(completed_at) = request_task.completed_at {
        task.completed_at = Set(completed_at);
    }

    save_active_task(&db, task).await?;

    Ok(())
}
```
[代码变动](https://github.com/CusterFun/rust-exercises/commit/9c2a197b87d29b386e19ad2bac8e88f1362da14b#diff-b7136d3e53a5279956cad1bc3652f43b2e12d7d0e3a83669358d0038b10a2a2b)
### 简化 `create_user.rs`

在 `src/queries/user_queries.rs` 中新增 `save_active_user` 函数

```rust
use axum::http::StatusCode;
use entity::{
    prelude::*,
    users::{self, Model as UsersModel},
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, TryIntoModel};

use crate::util::app_error::AppError;

pub async fn save_active_user(
    db: &DatabaseConnection,
    user: users::ActiveModel,
) -> Result<UsersModel, AppError> {
    let user = user.save(db).await.map_err(|err| {
        let error_message = err.to_string();
        if error_message
            .contains("duplicate key value violates unique constraint \"users_username_key\"")
        {
            AppError::new(
                StatusCode::BAD_REQUEST,
                "Username already taken, try again with a different user name",
            )
        } else {
            eprintln!("Error creating user: {:?}", &err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
        }
    })?;

    convert_active_to_model(user)
}

fn convert_active_to_model(active_user: users::ActiveModel) -> Result<UsersModel, AppError> {
    active_user.try_into_model().map_err(|err| {
        eprintln!("Error converting user active model to model: {err:?}");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
    })
}
```

将原文件 `src/api/users/create_user.rs`

```rust
use axum::{extract::State, http::StatusCode, Json};
use entity::tasks::Entity as Tasks;
use entity::{tasks, users};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, TryIntoModel,
};
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
        .map_err(|err| {
            let error_message = err.to_string();
            if error_message
                .contains("duplicate key value violates unique constraint \"users_username_key\"")
            {
                AppError::new(
                    StatusCode::BAD_REQUEST,
                    "Username already taken, try again with a different user name",
                )
            } else {
                eprintln!("Error creating user: {:?}", &err);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
            }
        })?
        .try_into_model()
        .map_err(|err| {
            eprintln!("Error converting user back into model: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
        })?;

    create_default_tasks_for_user(&db, &user).await?;

    Ok(Json(ResponseDataUser {
        data: ResponseUser {
            id: user.id,
            username: user.username,
            token: user.token.unwrap_or_default(),
        },
    }))
}

async fn create_default_tasks_for_user(
    db: &DatabaseConnection,
    user: &users::Model,
) -> Result<(), AppError> {
    let default_tasks = Tasks::find()
        .filter(tasks::Column::IsDefault.eq(Some(true)))
        .filter(tasks::Column::DeletedAt.is_null())
        .all(db)
        .await
        .map_err(|err| {
            eprintln!("Error getting default tasks: {:?}", err);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error applying default tasks to new account",
            )
        })?;
    Ok(for default_task in default_tasks {
        let task = tasks::ActiveModel {
            priority: Set(default_task.priority),
            title: Set(default_task.title),
            completed_at: Set(default_task.completed_at),
            description: Set(default_task.description),
            deleted_at: Set(default_task.deleted_at),
            user_id: Set(Some(user.id)),
            ..Default::default()
        };

        task.save(db).await.map_err(|err| {
            eprintln!("Error creating task from default: {:?}", err);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error saving new default task for user",
            )
        })?;
    })
}
```
简化为 

```rust
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
    let user = save_active_user(&db, new_user).await?;

    create_default_tasks_for_user(&db, &user).await?;

    Ok(Json(ResponseDataUser {
        data: ResponseUser {
            id: user.id,
            username: user.username,
            token: user.token.unwrap_or_default(),
        },
    }))
}
```

[代码变动](https://github.com/CusterFun/rust-exercises/commit/e80502fa7cd1a5d6efcb623fbdebdc0aadf8d411#diff-ee6af3e44180d400670d53d2b574ab6e4526b5a568a6d40c244b0db1b7f4f0bb)
### 简化 `create_default_tasks_for_user`

在 `src/queries/task_queries.rs` 中新增 `get_default_task` 函数

```rust
pub async fn get_default_task(db: &DatabaseConnection) -> Result<Vec<TasksModel>, AppError> {
    Tasks::find()
        .filter(tasks::Column::IsDefault.eq(Some(true)))
        .filter(tasks::Column::DeletedAt.is_null())
        .all(db)
        .await
        .map_err(|err| {
            eprintln!("Error getting default tasks: {:?}", err);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error applying default tasks to new account",
            )
        })
}
```

将原文件 `src/api/users/create_user.rs` 简化为 

```rust
use axum::{extract::State, Json};

use entity::{tasks, users};
use sea_orm::{DatabaseConnection, Set};
use types::user::{RequestCreateUser, ResponseDataUser, ResponseUser};

use crate::queries::task_queries::{get_default_task, save_active_task};
use crate::queries::user_queries::save_active_user;
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
    let user = save_active_user(&db, new_user).await?;

    create_default_tasks_for_user(&db, &user).await?;

    Ok(Json(ResponseDataUser {
        data: ResponseUser {
            id: user.id,
            username: user.username,
            token: user.token.unwrap_or_default(),
        },
    }))
}

async fn create_default_tasks_for_user(
    db: &DatabaseConnection,
    user: &users::Model,
) -> Result<(), AppError> {
    let default_tasks = get_default_task(db).await?;

    Ok(for default_task in default_tasks {
        let task = tasks::ActiveModel {
            priority: Set(default_task.priority),
            title: Set(default_task.title),
            completed_at: Set(default_task.completed_at),
            description: Set(default_task.description),
            deleted_at: Set(default_task.deleted_at),
            user_id: Set(Some(user.id)),
            ..Default::default()
        };

        save_active_task(&db, task).await?;
    })
}
```

[代码变动](https://github.com/CusterFun/rust-exercises/commit/b845ffaf2c9493212c3ac9d07de75a0bf165ca85#diff-ee6af3e44180d400670d53d2b574ab6e4526b5a568a6d40c244b0db1b7f4f0bb)

### 简化 `login.rs`

在 `src/queries/user_queries.rs` 中新增 `find_by_username` 函数

```rust
pub async fn find_by_username(
    db: &DatabaseConnection,
    username: String,
) -> Result<UsersModel, AppError> {
    Users::find()
        .filter(users::Column::Username.eq(username))
        .one(db)
        .await
        .map_err(|err| {
            eprintln!("Error getting user by username: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error loggin in")
        })?
        .ok_or_else(|| AppError::new(StatusCode::BAD_REQUEST, "bad username or password"))
}
```

将原文件 `src/api/users/login.rs`

```rust
use axum::{extract::State, http::StatusCode, Json};
use entity::{prelude::*, users};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set, TryIntoModel,
};
use types::user::{RequestCreateUser, ResponseDataUser, ResponseUser};

use crate::util::{app_error::AppError, hash::verify_password, jwt::create_token};

pub async fn login(
    State(db): State<DatabaseConnection>,
    State(secret): State<String>,
    Json(request_user): Json<RequestCreateUser>,
) -> Result<Json<ResponseDataUser>, AppError> {
    let user = Users::find()
        .filter(users::Column::Username.eq(request_user.username.as_str()))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error getting user for logging in: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error loggin in")
        })?;

    if let Some(user) = user {
        if !verify_password(&request_user.password, &user.password)? {
            return Err(AppError::new(
                StatusCode::UNAUTHORIZED,
                "bad username or password",
            ));
        }
        let token = create_token(&secret, user.username.clone())?;
        let mut user = user.into_active_model();
        user.token = Set(Some(token));
        let user = user
            .save(&db)
            .await
            .map_err(|err| {
                eprintln!("Error saving user token: {:?}", err);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error saving user token")
            })?
            .try_into_model()
            .map_err(|err| {
                eprintln!("Error converting model to active model: {:?}", err);
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error converting model to active model",
                )
            })?;
        let response = ResponseUser {
            id: user.id,
            username: user.username,
            token: user.token.unwrap_or_default(),
        };

        Ok(Json(ResponseDataUser { data: response }))
    } else {
        Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "bad username or password",
        ))
    }
}
```

简化为

```rust
use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{DatabaseConnection, IntoActiveModel, Set};

use types::user::{RequestCreateUser, ResponseDataUser, ResponseUser};

use crate::{
    queries::user_queries::{find_by_username, save_active_user},
    util::{app_error::AppError, hash::verify_password, jwt::create_token},
};

pub async fn login(
    State(db): State<DatabaseConnection>,
    State(secret): State<String>,
    Json(request_user): Json<RequestCreateUser>,
) -> Result<Json<ResponseDataUser>, AppError> {
    let user = find_by_username(&db, request_user.username).await?;

    if !verify_password(&request_user.password, &user.password)? {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "bad username or password",
        ));
    }
    let token = create_token(&secret, user.username.clone())?;
    let mut user = user.into_active_model();
    user.token = Set(Some(token));
    let user = save_active_user(&db, user).await?;

    let response = ResponseUser {
        id: user.id,
        username: user.username,
        token: user.token.unwrap_or_default(),
    };

    Ok(Json(ResponseDataUser { data: response }))
}
```
[代码变动](https://github.com/CusterFun/rust-exercises/commit/dc0f78fb6a0e5fb3d9f2d86029b3cb2e4c531a6d#diff-b5e833372dd39ee133868d12218c692a73c4ac09998fd7ec61ed61adc8e9c940)

### 简化 `logout.rs`

将原文件 `src/api/users/logout.rs`

```rust
use axum::{extract::State, http::StatusCode, Extension};
use entity::users;
use sea_orm::{ActiveModelTrait, DatabaseConnection, IntoActiveModel, Set};

use crate::util::app_error::AppError;

pub async fn logout(
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<users::Model>,
) -> Result<StatusCode, AppError> {
    let mut user = user.into_active_model();
    user.token = Set(None);
    user.save(&db).await.map_err(|err| {
        eprintln!("Error removing token: {err:?}");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    })?;
    Ok(StatusCode::OK)
}
```

简化为

```rust
use axum::{extract::State, http::StatusCode, Extension};
use entity::users;
use sea_orm::{DatabaseConnection, IntoActiveModel, Set};

use crate::{queries::user_queries::save_active_user, util::app_error::AppError};

pub async fn logout(
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<users::Model>,
) -> Result<StatusCode, AppError> {
    let mut user = user.into_active_model();
    user.token = Set(None);
    save_active_user(&db, user).await?;
    Ok(StatusCode::OK)
}
```

[代码变动](https://github.com/CusterFun/rust-exercises/commit/37a0b891152c87e906b1b9be0571bc7b687bfe60#diff-4ac4689c9e9c17852be3d67d135a611413354404d7faf3e1e83799a39fe18525)
