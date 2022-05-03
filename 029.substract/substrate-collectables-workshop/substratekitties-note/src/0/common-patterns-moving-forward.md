# 常见模式

## Rust 编译器是你的朋友

使用强类型编程语言（如 Rust）的众多优点之一是，编译器十分有用并且会对代码中出现的错误提供修复建议。通过[这篇文章](https://jvns.ca/blog/2018/01/13/rust-in-2018--way-easier-to-use/)来学习更多内容，关于Rust编译器如何帮助你。当然你还需要学习Rust，参考[Rust 官方书籍](https://doc.rust-lang.org/book/)。

## 更新你的 Runtime

在我们开始创建自定义 Substrate runtime 之前，你应该熟悉一些可以帮助你迭代和运行代码的模式。

你的 Substrate runtime 代码被编译为两个版本：

- WebAssembly（Wasm）image
- 标准二进制可执行文件

Wasm 文件被用来编译二进制文件的一部分，因此在构建可执行文件之前需要先编译 Wasm image。

模式应该是：

```bash
./scripts/build.sh       // 构建 Wasm
cargo build --release    // 构建 binary
```

此外，当你对节点进行更改时，之前旧版本节点生成的块仍然存在。你可能会注意到，当重启节点时，块只会从中断处继续生成。

但是，如果你对 runtime 的改动内容很多，那么可能需要使用以下命令清除链上先前所有的块：

```bash
./target/release/node-template purge-chain --dev
```

完成所有这些后，你将能够再次启动带有最近更改的节点：

```bash
./target/release/node-template --dev
```

记住这种模式，你会经常使用它。
