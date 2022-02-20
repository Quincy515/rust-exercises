```bash
cargo watch -q -c -w src/ -x run
```

- https://github.com/dwisulfahnur/actix-graphql-demo
- https://github.com/dwisulfahnur/actix-todoApp

## 文章

[GraphQL] 构建 Rust 异步 GraphQL 服务：基于 tide + async-graphql + mongodb

- [（1）- 起步及 crate 选择](<https://blog.budshome.com/budshome/gou-jian-rust-yi-bu-graphql-fu-wu-:ji-yu-tide-+-async-graphql-+-mongodb(1)--qi-bu-ji-crate-xuan-ze>)
- [（2）- 查询服务](<https://blog.budshome.com/budshome/gou-jian-rust-yi-bu-graphql-fu-wu-:ji-yu-tide-+-async-graphql-+-mongodb(2)--cha-xun-fu-wu>)
- [（3）- 重构](<https://blog.budshome.com/budshome/gou-jian-rust-yi-bu-graphql-fu-wu-:ji-yu-tide-+-async-graphql-+-mongodb(3)--zhong-gou>)

## Axum

- ☆ axum+sqlx [Z4RX/axum_jwt_example](https://github.com/Z4RX/axum_jwt_example)
- [linux-china/axum-demo](https://github.com/linux-china/axum-demo)
- ☆ A opensource community written by rust axum+sqlx [Whatsoo/whatsoo](https://github.com/Whatsoo/whatsoo)
- ☆ RESTful API template built with Rust lang. It uses [MongoDB](https://docs.mongodb.com/) database and [Axum](https://github.com/tokio-rs/axum) HTTP framework.[ndelvalle](https://github.com/ndelvalle)/**[rustapi](https://github.com/ndelvalle/rustapi)** 
- axum+rbatis [新手上路，用 rust 撸了一个 restful api 服务的 demo - 大家的项目](https://github.com/liyue201/rust-rest-demo)
- axum+[Skytable](https://github.com/skytable/skytable) jotsy - 一个自托管，免费和开源（Apache-2.0）的笔记程序，使用 Skytable，Axum 和 Tokio 构建。jotsy 最重要的目标是简单，专注于最重要的事情，记笔记。[文章链接](https://www.reddit.com/r/rust/comments/skizcp/jotsy_a_selfhosted_notes_app_powered_by_skytable/)，https://www.reddit.com/r/rust/comments/skizcp/jotsy_a_selfhosted_notes_app_powered_by_skytable/ [Github 链接](https://github.com/ohsayan/jotsy)，https://github.com/ohsayan/jotsy

## Actix

- [sansx/cn_poems_api](https://github.com/sansx/cn_poems_api)
- [chuancoder/rust-web](https://gith
  ub.com/chuancoder/rust-web)
- [SakaDream/actix-web-rest-api-with-jwt](https://github.com/SakaDream/actix-web-rest-api-with-jwt)
- [auth0-blog/actix-diesel-auth](https://github.com/auth0-blog/actix-diesel-auth)
- [svenstaro/rust-web-boilerplate](https://github.com/svenstaro/rust-web-boilerplate)

`#[validate(required)]`

- [JavaHello/rust-web-example](https://github.com/JavaHello/rust-web-example)
- [ericpubu/aoc-2020](https://github.com/ericpubu/aoc-2020)
- [beemstream/profile-service](https://github.com/beemstream/profile-service)

---

[Building Powerful GraphQL Servers with Rust](https://dev.to/open-graphql/building-powerful-graphql-servers-with-rust-3gla)

[Rust GraphQL Example](https://github.com/iwilsonq/rust-graphql-example)

It is based on async-graphql, actix-web, diesel, powered by Rust.
[heyrict/cindy-next-rust](https://github.com/heyrict/cindy-next-rust)

[openmsupply/remote-server](https://github.com/openmsupply/remote-server)

[biluohc/actixweb-sqlx-jwt](https://github.com/biluohc/actixweb-sqlx-jwt)

[WKHAllen/GreenPollAPI](https://github.com/WKHAllen/GreenPollAPI)



## poem

- poem+sqlx [SIT-kite](https://github.com/SIT-kite)/**[kite-server](https://github.com/SIT-kite/kite-server)**
- poem+sea-orm [lingdu1234](https://github.com/lingdu1234)/**[poem_admin](https://github.com/lingdu1234/poem_admin)**
  - 完成的功能
    1. 用户管理：用户是系统操作者，该功能主要完成系统用户配置。
    2. 部门管理：配置系统组织机构（公司、部门、小组），树结构展现支持数据权限。
    3. 岗位管理：配置系统用户所属担任职务。
    4. 菜单管理：配置系统菜单，操作权限，按钮权限标识等。
    5. 角色管理：角色菜单权限分配、设置角色按机构进行数据范围权限划分。
    6. 字典管理：对系统中经常使用的一些较为固定的数据进行维护。
    7. 登录日志：系统登录日志记录查询包含登录异常。
    8. 在线用户：当前系统中活跃用户状态监控。
    9. 定时任务：在线（添加、修改、删除)任务调度包含执行结果日志。\
  - 待完成
    1. 权限控制完善，数据权限，角色，部门切换
    2. 系统监控
    3. 无用代码清理
    4. 。。。
