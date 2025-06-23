# RustNetPerfFFI

## 目录结构
- src/lib.rs: FFI 绑定声明
- build.rs: 构建脚本，链接 C++ 静态库
- examples/simple.rs: Rust 示例，调用 echo_server

## 构建与运行

1. 确保已在项目根目录下编译生成 build/libnet-perf.a
2. 进入 RustNetPerfFFI 目录，构建并运行示例：

```sh
cd RustNetPerfFFI
cargo run --example simple
```

如需调用 ping-pong client，可在 Rust 里添加对应调用。 