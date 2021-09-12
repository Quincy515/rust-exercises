# 利用 Tokio 实现一个高新能 Mini Http Server|Web 框架建议选用 Axum

目的
1. 实战：学会从 0 开始设计架构短链服务
2. Http Server 本质
3. 了解 Rust Web 框架生态，技术选型

## 用 Tokio 实现一个短链系统
涉及的知识点
MySQL、Redis、RPC

学完后可以举一反三，用 Rust 构建更多的应用

## 回顾 Tokio 是什么
Tokio 是一个 Rust 异步运行时库，底层基于 epoll/kqueue 这样的跨平台多路复用 IO 以及 event loop，目前正在支持 io_uring。它的 scheduler 和 Erlang/Go 实现的 N:M threads 类型，线程会执行 Task，可以充分利用多核。Task 是 Rust 基于 Future 抽象出的一种绿色线程，因为不需要预先分配多余的栈内存，可以创建大量 task，很适合做 IO 密集型应用。

## 利用 Tokio 实现一个高新能 Mini Http Server

