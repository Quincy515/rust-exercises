### 安装 tailwindcss 和 daisyui

```shell
npm install -D tailwindcss daisyui@latest
```

- https://leptos-rs.github.io/leptos/
- https://daisyui.com/docs/install/?lang=zh_hant
- https://github.com/LemmyNet/lemmy-ui-leptos
- https://github.com/leptos-rs/leptos/blob/main/examples/tailwind_csr_trunk/README.md

### 在项目根目录下执行

```shell
npx tailwindcss init
```

修改 tailwind.config.js

```js
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],
  daisyui: {
    logs: false, // Need to disable logs in order for build to succeed. See https://github.com/leptos-rs/cargo-leptos/issues/136
  },
};
```

新建 style/input.css

```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```

最后修改 index.html

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <link data-trunk rel="rust" data-wasm-opt="z" />
    <link data-trunk rel="icon" type="image/ico" href="/public/favicon.ico" />
    <link data-trunk rel="css" href="/style/output.css" />
    <title>Leptos • Tutorial</title>
</head>
<body>
    
</body>
</html>
```

在终端执行 

```shell
npx tailwindcss -i ./style/input.css -o ./style/output.css --watch
```

打开另一个终端，执行

```shell
trunk serve --open
```

### 此时文件详情

```rust
// src/main.rs

mod app;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::logging::log!("csr mode - mounting to body");

    leptos::mount_to_body(|| leptos::view! { <app::App /> });
}
```

```rust
// src/app.rs

use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <div class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">"Welcome to Leptos with Tailwind"</h2>
            <p class="px-10 pb-10 text-left">"Tailwind will scan your Rust files for Tailwind class names and compile them into a CSS file."</p>
            <button
                class="btn"
                class:red=move||count()%2==1
                on:click=move |_| set_count.update(|n| *n += 1)
            >
                "Click me: "
                {move || count()}
            </button>
        </div>
    }
}
```

```shell
┌─ Cargo.toml
├─ package.json
├─ README.md
├─ tailwind.config.js
├─ index.html
│  ┌─ main.rs
│  ├─ app.rs
├─ src
│  ┌─ favicon.ico
├─ public
│  ┌─ tailwind.css
│  ├─ output.css
├─ style
├─ package-lock.json
├─ Cargo.lock
├─ dist
```