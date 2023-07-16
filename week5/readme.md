# week5

1. cargo lib 包内定义二进制目标

如果你在 src/bin 目录下有 .rs 文件，Cargo会默认将这些文件识别为独立的二进制应用程序的主入口。每个 .rs 文件都会编译成一个单独的可执行文件。文件名将用作生成的可执行文件的名称。

如果声明复杂的path 关联，可以使用 [[bin]] 声明,

比如:

```rust
[[bin]]
name = "my_app"
path = "src/main.rs"

[[bin]]
name = "my_worker"
path = "src/worker/main.rs"
```

2. 格式化输出

使用 format 宏，可以完成字符串的padding, 其中分为几种padding方式

```rust
println!("{:10}", "hello");     
println!("{:*<10}", "hello");
println!("{:*>10}", "hello");
println!("{:*^30}", "hello world");
```

其中 10 为长度, ><^ 表示对齐方向，分别为 左对齐，右对齐和居中对齐。
另外还可以自定义填充字符。

3. 日期相关

使用chrono 处理日期格式化时区的问题。

```rust
pub fn current_timestamp() -> u64 {
    let local: DateTime<Local> = Local::now();
    local.timestamp_millis() as u64
}
pub fn now_string() -> String {
    let local: DateTime<Local> = Local::now(); // e.g. `2014-11-28T21:45:59.324310806+09:00`
    local.to_string()
}
```

4. 变量struct 可以格式化，需要实现 Display trait

```rust
    impl Display for Network {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "[{}, {}]", self.to_string(), self.get_gateway())
        }
    }
```

5. 打开文档链接，可用于 demo  day 介绍

cargo doc --open