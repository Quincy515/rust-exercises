# 所有权系统

Ownership System 包括

- 所有权 Owenership
- 借用 Borrowing
- 生命周期 Lifetimes

Rust 编译器在编译时会根据所有权系统的一系列规则进行检查，所有的分析与检查都是在编译时完成的。因此所有权系统拥有零或者极小的运行时开销。

## 栈于堆对应的语义

### 栈内存

