/// 详细的栈调用信息
/// 默认情况下，栈调用只会展示最基本的信息:
/// thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
/// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
/// 但是有时候，我们还希望获取更详细的信息:


/// 3. 
/**
## 填空以打印全部的调用栈
## 提示: 你可以在之前的默认 panic 信息中找到相关线索
$ __ cargo run
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `[97, 98, 99]`,
 right: `[96, 97, 98]`', src/main.rs:3:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/std/src/panicking.rs:498:5
   1: core::panicking::panic_fmt
             at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/panicking.rs:116:14
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed
             at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/panicking.rs:154:5
   4: study_cargo::main
             at ./src/main.rs:3:5
   5: core::ops::function::FnOnce::call_once
             at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/ops/function.rs:227:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
*/