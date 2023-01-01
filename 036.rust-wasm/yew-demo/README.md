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



