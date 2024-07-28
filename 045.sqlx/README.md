https://www.postgresql.org/download/

```shell
brew install postgresql@16

If you need to have postgresql@16 first in your PATH, run:
  echo 'export PATH="/usr/local/opt/postgresql@16/bin:$PATH"' >> ~/.zshrc

For compilers to find postgresql@16 you may need to set:
  export LDFLAGS="-L/usr/local/opt/postgresql@16/lib"
  export CPPFLAGS="-I/usr/local/opt/postgresql@16/include"

For pkg-config to find postgresql@16 you may need to set:
  export PKG_CONFIG_PATH="/usr/local/opt/postgresql@16/lib/pkgconfig"

To start postgresql@16 now and restart at login:
  brew services start postgresql@16
Or, if you don't want/need a background service you can just run:
  LC_ALL="C" /usr/local/opt/postgresql@16/bin/postgres -D /usr/local/var/postgresql@16
```

登录 psql 输入 `psql postgres`

查看数据库列表 `\l`

进入数据库 `\c postgres`

安装 **`pgcli`** https://www.pgcli.com/docs

```shell
brew install pgcli
```

直接使用 `pgcli postgres` 进入数据库，可以更加方便的使用，美化输出，自动补全等功能。

```shell
cargo install sql-cli --no-default-features --features rustls --features postgres
```

创建 chat 数据库

```shell
dropdb chat
createdb chat
pgcli chat
```
第一次运行 sqlx migrate 使用命令创建 migrations 目录

```shell
sqlx migrate add initial
```

在创建的 `migrations/20240728023616_initial.sql` 文件中添加数据库表结构

```sql
-- create user table
CREATE TABLE IF NOT EXISTS users(
    id bigserial PRIMARY KEY,
    ws_id bigint NOT NULL,
    fullname varchar(64) NOT NULL,
    email varchar(64) NOT NULL,
    -- hashed argon2 password, length 97
    password_hash varchar(97) NOT NULL,
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP
);
```

首先添加 `.env` 文件然后执行 `sqlx migrate run` 进行数据库迁移

```.env
DATABASE_URL=postgres://postgres@localhost/chat
```

```shell
cargo add sqlx --features postgres --features runtime-tokio --features tls-rustls --features chrono
```

安装 `jwt-simple` 默认会使用 `C++` 的 `boring` 库，编译会比较慢，可以使用 `pure-rust` 版本

```shell
cargo add jwt-simple --no-default-features --features optimal --feautres pure-rust
```

使用非对称 ED22519 签名算法，生产的 token 更短，public key 可以给客户端验证 token

generating public ed25519 key with openssl

```shell
openssl genpkey -algorithm ed25519 -out private.pem
openssl pkey -in private.pem -pubout -out public.pem
```


