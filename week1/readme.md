# rust 学习笔记

1. 基本类型使用 u32 u64 i32 f32 ， str String 等
2. 泛型简单实现 Vector HashMap
3. 选择 循环结构 if for loop while
4. 模块定义及引用 mod pub
5. 面向对象 struct 类型 impl 方法定义
6. 发布程序到 crates.io

```shell
cargo new 
cargo login # 需要一个token 
cargo publish # 需要 Cargo.toml 中有足够的说明
cargo install # 安装crates.io 
```

7. 其他

- json 格式化 serde_json 和 serde
- 终端颜色 ansi_term
- ed25519 签名数据及验证 (注意ed25519-dalek 依赖的版本号 rand rand_core)
- 通过环境变量传递固定私钥 :  secret_key=599f6ec8dfc486cffeebb8ddab1e5c23913b16fbaf87388c68fdf5cfcd80bf4e week1