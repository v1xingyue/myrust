# week4

## 错误处理

错误按照级别分几种

1. 类型错误，此类错误在编译初期解决
2. 可能返回None 的 用 Option 来封装。 Option 和 Some 配对使用。处理返回结果用 match
3. 如果你还想把error 传回调用者，那么使用 Result 来封装。定义完成以后，可以使用 Ok Err 返回对应的信息

类似下边的例子:

```rust
fn return_error(a: u64, b: u64) -> Result<u64, String> {
    if b != 0 {
        Ok(a / b)
    } else {
        Err(String::from("b can't be 0 "))
    }
}
```

Result 的设计，还是比较有意思的,用一个变量完成了结果和error
match 结果 解决了其他语言中的 if判断

4. 断言 理性防御
才往下执行，否则将会报错.
分三种 
assert 参数必须为真，否则报错
assert_eq 两个参数必须相等
assert_ne 两个参数必须不相等

5. 最后一种，遇到实在不能处理的错误，那么直接触发 panic 即可。 

## 关于 cargo 项目

Cargo.toml 

[workspace]
members = ["mybase","mylib"]
是用来定义发布的包的

可以用来声明本地的依赖
[dependencies]
mylib = {path="./mylib"}

关于测试 
cargo test 测试当前package
cargo test --workspace 测试 workspace中定义的member ，并包含当前package 


关于集成测试

1. 集成测试一般只适用于 lib package , 同时，只能调用lib.rs 中定义的module。即作为一个整体来执行。
2. 在 Rust 中，crate 关键字只能在库（lib）包的根模块中使用，它用于引用当前包的内容。而在 tests 目录中，测试文件是作为独立的模块执行的，无法直接访问 crate 关键字。
3. 如果你需要强行测试，可以使用文件路径指定module 

```rust
#[path = "../src/myinfo.rs"]
mod myinfo;
```

