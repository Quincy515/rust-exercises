RESTful API in Sync & Async Rust

[原文](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md) 来源是 github 上的 pretzelhammer, 他在 github 上开源了自己的 rust 博客, 上面的文章质量很高。 本文介绍了如何在 Rust 中为一个虚构的看板风格的项目管理应用程序实现一个 RESTful API 服务器。

[rustmagazine.github.io](https://rustmagazine.github.io/rust_magazine_2021/chapter_5/learn.html#restful-api-in-sync--async-rust) 

[构建 Rust 异步 GraphQL 服务：基于 tide + async-graphql + mongodb（1）- 起步及 crate 选择 - 云上于天 - 芽之家 - budshome.com](https://blog.budshome.com/budshome/gou-jian-rust-yi-bu-graphql-fu-wu-:ji-yu-tide-+-async-graphql-+-mongodb(1)--qi-bu-ji-crate-xuan-ze)

## Intro

## General

### Project Setup

```bash
cargo new kanban
```

#### 工具类 crate 安装

工程创建完成后，我们即可以进入开发环节了。开发中，一些工具类 crate 可以起到“善其事”的作用，我们需要先进行安装。

- cargo-edit，包含 `cargo add`、`cargo rm`，以及 `cargo upgrade`，可以让我们方便地管理 crate。
- cargo-watch，监视项目的源代码，以了解其更改，并在源代码发生更改时，运行 Cargo 命令。

好的，我们安装这 2 个 crate。

```bash
cargo install cargo-edit
cargo install cargo-watch
```

> 安装依赖较多，如果时间较长，请[配置 Rust 工具链的国内源](https://rust-guide.budshome.com/3-env/3.1-rust-toolchain-cn.html)。

#### 添加依赖 crate

接着，我们需要添加开发所需依赖项。依赖项的添加，我们不用一次性全部添加，我们根据开发需要，一步步添加。首先，从后端工程开始。

后端工程中，我们提供 GraphQL 服务，需要依赖的基本 crate 有 tide、async-std、async-graphql、mongodb，以及 bson。我们使用 `cargo add` 命令来安装，其将安装最新版本。

```Bash
cd kanban
cargo add tide async-std async-graphql mongodb bson
```

### Loading Environment Variables w/dotenv

#### 配置信息的存储和获取

让我们设想正式生产环境的应用场景：

- 服务器地址和端口的变更可能；
- 服务功能升级，对用户暴露 API 地址的变更可能。如 rest api，graphql api，以及版本升级；
- 服务站点密钥定时调整的可能；
- 服务站点安全调整，jwt、session/cookie 过期时间的变更可能。

显然易见，我们应当避免每次变更调整时，都去重新编译一次源码——并且，大工程中，Rust 的编译速度让开发者注目。更优的方法是，将这些写入到配置文件中。或许上述第 4 点无需写入，但是文件存储到加密保护的物理地址，安全方面也有提升。

当然，实际的应用场景或许有更合适有优的解决方法，但我们先基于此思路来设计。Rust 中，`dotenv` crate 用来读取环境变量。取得环境变量后，我们将其作为静态或者惰性值来使用，静态或者惰性值相关的 crate 有 `lazy_static` 和 `once_cell` 等，都很简单易用。此示例中，我们使用 `lazy_static`。

#### 创建 `.env`，添加读取相关 crate

增加这 2 个 crate，并且在 `backend` 目录创建 `.env` 文件。

```Bash
cargo add dotenv lazy_static
touch .env
```

在 `.env` 文件中，写入如下内容：

```.env
# .env

LOG_LEVEL=INFO
LOG_FILE=server.log
DATABASE_URL=postgres://postgres@localhost:5432/postgres 
```

Updated main file which uses `dotenv`:

```rust
// src/main.rs

type StdErr = Box<dyn std::error::Error>;

fn main() -> Result<(), StdErr> {
    // loads env variables from .env
    dotenv::dotenv()?;

    // example
    assert_eq!("INFO", std::env::var("LOG_LEVEL").unwrap());

    Ok(())
}
```

### Handling Dates & Times w/chrono

crates `chrono` handling dates & times is chrono.

```bash
cargo add chrono
```

### Logging w/fern

crates `log`  `fern` 

```bash
cargo add log fern
```

Log is Rust's logging facade library. It provides the high-level logging API but we still need to pick an implementation, and the implementation we're going to use is the fern crate. Fern allows us to easily customize the logging format and also chain multiple outputs so we can log to stderr and a file if we wanted to. After adding log and fern let's encapsulate all of the logging configuration and initialization into its own module:

```rust
// src/logger.rs

use std::env;
use std::fs;
use log::{debug, error, info, trace, warn};

pub fn init() -> Result<(), fern::InitError> {
    // pull log level from env
    let log_level = env::var("LOG_LEVEL").unwrap_or("INFO".into());
    let log_level = log_level
        .parse::<log::LevelFilter>()
        .unwrap_or(log::LevelFilter::Info);

    let mut builder = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                chrono::Local::now().format("%H:%M:%S"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log_level)
        // log to stderr
        .chain(std::io::stderr());

    // also log to file if one is provided via env
    if let Ok(log_file) = env::var("LOG_FILE") {
        let log_file = fs::File::create(log_file)?;
        builder = builder.chain(log_file);
    }

    // globally apply logger
    builder.apply()?;

    trace!("TRACE output enabled");
    debug!("DEBUG output enabled");
    info!("INFO output enabled");
    warn!("WARN output enabled");
    error!("ERROR output enabled");

    Ok(())
}
```

And then add that module to our main file:

```rust
// src/main.rs

+mod logger;

type StdErr = Box<dyn std::error::Error>;

fn main() -> Result<(), StdErr> {
    dotenv::dotenv()?;
+   logger::init()?;

    Ok(())
}
```

If we run the program now, since `INFO` is the default logging level, here's what we'd see:

```bash
$ cargo run
[08:36:30][kanban::logger][INFO] INFO output enabled
[08:36:30][kanban::logger][WARN] WARN output enabled
[08:36:30][kanban::logger][ERROR] ERROR output enabled
```

### JSON Serialization w/serde

crates `serde` `serde_json`

```bash
cargo add serde serde_json
```

```toml
# Cargo.toml

[package]
name = "kanban"
version = "0.1.0"
edition = "2018"

[dependencies]
dotenv = "0.15"
- chrono = "0.4"
+ chrono = { version = "0.4", features = ["serde"] }
log = "0.4"
fern = "0.6"
+ serde = { version = "1.0", features = ["derive"] }
+ serde_json = "1.0"
```

### Domain Modeling

```rust
// src/models.rs

// for GET requests

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Board {
    pub id: i64,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub id: i64,
    pub board_id: i64,
    pub description: String,
    pub status: Status,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Todo,
    Doing,
    Done,
}

#[derive(serde::Serialize)]
pub struct BoardSummary {
    pub todo: i64,
    pub doing: i64,
    pub done: i64,
}

// for POST requests

#[derive(serde::Deserialize)]
pub struct CreateBoard {
    pub name: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCard {
    pub board_id: i64,
    pub description: String,
}

// for PATCH requests

#[derive(serde::Deserialize)]
pub struct UpdateCard {
    pub description: String,
    pub status: Status,
}
```

And the updated main file:

```rust
// src/main.rs

mod logger;
+mod models;

type StdErr = Box<dyn std::error::Error>;

fn main() -> Result<(), StdErr> {
    dotenv::dotenv()?;
    logger::init()?;

    Ok(())
}
```

## Sync Implementation

### SQL Schema Migrations w/diesel-cli

crates `diesel-cli`

```bash
cargo install diesel_cli
```

If the above command doesn't work at first, it's likely because we don't have all the development libraries for all of diesel-cli's supported databases. Since we're just using PostgreSQL, we can make sure the development libraries are installed with these commands:

```bash
# macOS
brew install postgresql

# ubuntu
apt-get install postgresql libpq-dev
```

And then we can tell cargo to only install diesel-cli with support for PostgreSQL:

```bash
cargo install diesel_cli --no-default-features --features postgres
```

Once we have diesel-cli installed we can use it to create new migrations and execute pending migrations. diesel-cli figures out which DB to connect to by checking the `DATABASE_URL` environment variable, which it will also load from an `.env` file if one exists in the current working directory.

Assuming the DB is currently running and a `DATABASE_URL` environment variable is present, here's the first diesel-cli command we'd run to bootstrap our project:

```
diesel setup
```

With this diesel-cli creates a `migrations` directory where we can generate and write our DB schema migrations. Let's generate our first migration:

```
diesel migration generate create_boards
```
