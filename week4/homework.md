## 你认为的错误处理逻辑应该是怎样的？

按照情况分为如下五种:

1. 类型错误，此类错误在编译初期解决
2. 可能返回None 的 用 Option 来封装。 Option 和 Some 配对使用。处理返回结果用 match
3. 如果你还想把error 传回调用者，那么使用 Result 来封装。定义完成以后，可以使用 Ok Err 返回对应的信息
4. 断言 理性防御
5. 最后一种，如果实在处理不了了，那就 panic 

## 以一个工作空间的形式创建一个项目，并在其下创建多个模块，并把它们关联起来，使之能够正常编译运行

关键还是toml 文件

```rust

[package]
name = "app"
version = "0.1.0"
edition = "2021"

[workspace]
members = ["mybase","mylib"]

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
mylib = {path="./mylib"}

```

## 尝试编写单元测试和集成测试

1. 单元测试，在模块内部声明:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

```

2. 关于集成测试

集成测试一般只适用于 lib package , 同时，只能调用lib.rs 中定义的module。即作为一个整体来执行。

在 Rust 中，crate 关键字只能在库（lib）包的根模块中使用，它用于引用当前包的内容。而在 tests 目录中，测试文件是作为独立的模块执行的，无法直接访问 crate 关键字。

如果你需要强行测试，可以使用文件路径指定module 

```rust
#[path = "../src/myinfo.rs"]
mod myinfo;
```

