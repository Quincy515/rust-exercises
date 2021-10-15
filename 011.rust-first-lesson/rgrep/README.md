做的小工具是 rgrep，它是一个类似 grep 的工具。如果你是一个 \*nix 用户，那大概率使用过 grep 或者 ag 这样的文本查找工具。

grep 命令用于查找文件里符合条件的字符串。如果发现某个文件的内容符合所指定的字符串，grep 命令会把含有字符串的那一行显示出；若不指定任何文件名称，或是所给予的文件名为 -，grep 命令会从标准输入设备读取数据。

我们的 rgrep 要稍微简单一些，它可以支持以下三种使用场景：

首先是最简单的，给定一个字符串以及一个文件，打印出文件中所有包含该字符串的行：

```shell
$ rgrep Hello a.txt
55: Hello world. This is an exmaple text
```

然后放宽限制，允许用户提供一个正则表达式，来查找文件中所有包含该字符串的行：

```shell
$ rgrep Hel[^\\s]+ a.txt
55: Hello world. This is an exmaple text
89: Help me! I need assistant!
```

如果这个也可以实现，那进一步放宽限制，允许用户提供一个正则表达式，来查找满足文件通配符的所有文件（你可以使用 globset 或者 glob 来处理通配符），比如：

```shell
$ rgrep Hel[^\\s]+ a*.txt
a.txt
    55:1 Hello world. This is an exmaple text
    89:1 Help me! I need assistant!
    5:6  Use `Help` to get help.
abc.txt:
    100:1 Hello Tyr!
```

其中，冒号前面的数字是行号，后面的数字是字符在这一行的位置。

给你一点小提示。

- 对于命令行的部分，你可以使用 clap3 或者 structopt，也可以就用 env.args()。
- 对于正则表达式的支持，可以使用 regex。
- 至于文件的读取，可以使用 std::fs 或者 tokio::fs。你可以顺序对所有满足通配符的文件进行处理，也可以用 rayon 或者 tokio 来并行处理。
- 对于输出的结果，最好能把匹配的文字用不同颜色展示。

1. 最简单的

```rust
use std::error::Error;

use clap::{AppSettings, Clap};
use colored::Colorize;
use tokio::fs;

#[derive(Clap)]
#[clap(version = "1.0", author = "Custer<custer@email.cn>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    find: String,
    path: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 1. 解析参数
    let opts: Opts = Opts::parse();
    let find = opts.find;
    let path = opts.path;
    let length = find.len();

    // 2. 读取文件
    let contents = fs::read_to_string(path).await?;

    // 3. 匹配字符串
    for (row, line) in contents.lines().enumerate() {
        if let Some(col) = line.find(&find) {
            println!(
                "{}:{} {}{}{}",
                row + 1,
                col + 1,
                &line[..col],
                &line[col..col + length].red().bold(),
                &line[col + length..]
            );
        }
    }
    Ok(())
}
```

2. 允许用户提供一个正则表达式，来查找文件中所有包含该字符串的行

```rust
    // 3. 匹配字符串
    for (row, line) in contents.lines().enumerate() {
        if let Some(re) = Regex::new(find.as_str()).unwrap().find(line) {
            let start = re.start();
            let end = re.end();
            println!(
                "{}:{} {}{}{}",
                row + 1,
                start + 1,
                &line[..start],
                &line[start..end].red().bold(),
                &line[end..]
            );
        }
    }
```

3. 允许用户提供一个正则表达式，来查找满足文件通配符的所有文件(好像并不需要使用 globset 或者 glob 就可以处理通配符？）

```rust
...
struct Opts {
    find: String,
    #[clap(multiple_values = true)]
    paths: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 1. 解析参数
    let opts: Opts = Opts::parse();
    let find = opts.find.as_str();
    let paths = opts.paths;

    // 2. 循环读取匹配到的文件
    for path in paths {
        println!("{:?}", path);
        let contents = fs::read_to_string(path).await?;

        // 3. 匹配字符串
        ...
    }
    Ok(())
}
```

运行

```shell
cargo run --quiet -- "Re[^\\s]+" src/*.rs  
```