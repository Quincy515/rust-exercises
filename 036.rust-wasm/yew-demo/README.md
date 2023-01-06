## yew基本配置、创建最简单界面

https://yew.rs/docs/next/getting-started/introduction

```shell
rustup target add wasm32-unknown-unknown
cargo install trunk
cargo install wasm-bindgen-cli
```

`cargo` 依赖

```toml
[dependencies]
yew = {version = "0.20.0", features = ["csr"]}
```

在 `main.rs` 中添加 

```rust
fn main() {
        yew::Renderer::<>::new().render();
}
```

在 `src` 文件夹下新建组件目录 `components` 和 `mod.rs`、`test.rs` 文件

在 `mod.rs` 中添加 

```rust
pub mod test;
```

在 `test.rs` 中添加

```rust
use yew::prelude::*;

pub enum Msg {}

// 组件对象
pub struct TestComp {
    name: String,
}

impl Component for TestComp {
    type Message = Msg;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        Self {
            name: String::from("test"),
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
            <h1>{&self.name}</h1>
            </div>
        }
    }
}
```

修改 `main.rs`

```rust
mod components;
use components::test::TestComp;

fn main() {
    yew::Renderer::<TestComp>::new().render();
}
```

在根目录下新建 `index.html` 

```html
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8" />
        <title>myapp</title>
    </head>
</html>
```

此时

```shell
├── Cargo.lock
├── Cargo.toml
├── README.md
├── index.html
├── src
│   ├── components
│   │   ├── mod.rs
│   │   └── test.rs
│   └── main.rs
```

执行 

```shell
cargo build
```

运行服务

```shell
trunk serve
```

打开浏览器  http://127.0.0.1:8080

## 添加按钮、触发事件、JS交互（wasm-bindgen）

https://github.com/rustwasm/wasm-bindgen

```toml
[dependencies]
yew = {version = "0.20.0", features = ["csr"]}
wasm-bindgen = "0.2"
```

新建文件夹 `helper` 新建文件 `mod.rs` 和 `js.rs` 

在 `mod.rs` 中添加

```rust
pub mod js;
```

在 `js.rs` 中添加 

```rust
// alert('xxx')

use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}
```

在 `main.rs` 中添加

```rust
mod components;
mod helper; // <- new
use components::test::TestComp;

fn main() {
    yew::Renderer::<TestComp>::new().render();
}
```

修改 `test.rs` 

```rust
use crate::helper::js;
use yew::prelude::*;

pub enum Msg {
    TestClick,
}

// 组件对象
pub struct TestComp {
    name: String,
}

impl Component for TestComp {
    type Message = Msg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            name: String::from("test"),
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::TestClick => {
                js::alert("按钮点击");
                true
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div>
            <button onclick={link.callback(|_|Msg::TestClick)}>{"按钮"}</button>
            <h1>{&self.name}</h1>
            </div>
        }
    }
}
```

此时运行 

```shell
trunck serve
```

访问 http://127.0.0.1:8080/ 点击按钮可以看到 `alert` 弹出框

## 使用 web-sys 进行 js 交互

https://rustwasm.github.io/wasm-bindgen/web-sys/index.html

基于 wasm-bindgen 提供原生的导入，文档 https://docs.rs/web-sys/latest/web_sys/

加入依赖

```toml
[dependencies]
wasm-bindgen = "0.2"
yew = {version = "0.20.0", features = ["csr"]}

[dependencies.web-sys]
features = [
  'Document',
  'Element',
  'HtmlElement',
  'Node',
  'Window',
]
version = "0.3"
```

这样就可以把 `js.rs` 原写法

```rust
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}
```

简化写成 

```rust
use web_sys::window;

// window.alert
pub fn alert(str: &str) {
    window().unwrap().alert_with_message(str).unwrap();
}
```

## 在 wasm 中 http 请求的基本代码

http://github.com/rustwasm/gloo

glow 是一个库集合（工具包），不是一个具体的框架

web-sys/js-sys 直接使用非常不方便，因此 gloo 提供了原始绑定的包装器

- 我们主要使用 gloo-net https://github.com/rustwasm/gloo/tree/master/crates/net
- yew 官网推荐 https://yew.rs/docs/tutorial#fetching-data-using-external-rest-api

在依赖中添加 

```toml
[dependencies]
gloo-net = "0.2"
serde = { version = "1.0", features = ["derive"] }
wasm-bindgen-futures = "0.4"
```

`serde` 是用来做序列化和反序列化的

`wasm-bindgen-futures` 主要是实现前端 (js) 中的 Promise

首先在 `js.rs` 中添加 `log` 函数

```rust
use web_sys::console;

...

// console.log(xxx)
pub fn log(obj: &wasm_bindgen::JsValue) {
    console::log_1(obj);
}
```

