# homework week1

## rustup 是什么，如何用它来管理 Rust 版本？

rustup 为 rust 的版本管理工具， 类似于 nvm , pyenv 这些工具。

```bash
rustup show    # 检查安装的版本
rustup install $name  # 安装指定版本
rustup uninstall $name  # 卸载指定版本
rustup default $name # 设置默认版本，指定版本
```

## Rust 中&str 和 String 的区别是什么，每个应该在什么时候使用？

str 为定长的字符串，存储在栈上，为rust 原生支持类型。 

String 为扩展结构体，本质是一个 vector<u8> ， 长度可以改变，存储在堆上，同时支持的方法更多。 

## Rust 中的泛型类型是什么，你可以自己写几个例子吗？

泛型为定义结构体，函数，方式的时候，可以传递一个类型参数T。同时，可以实现部分，可以根据不同的类型T 提供不同的实现。可以用来设计更加通用的数据类型。

泛型可以用类设计通用的容器数据结构，内部可以包含多样性的数据。

以下是对Wrapper 的一个简单实现:

```rust
struct Wrapper<T> {
    data: T,
}

impl<T> Wrapper<T> {
    fn hello(&self) -> &T {
        &self.data
    }
    fn hello_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

fn main(){
   	let mut wrapper = Wrapper {
        data: "this in wrapper message ..".to_string(),
    };

    let data = Wrapper::hello(&wrapper);
    println!("{}", data);

    let m: &mut String = Wrapper::hello_mut(&mut wrapper);
    *m = String::from("this message chagned from outsice ...");

    println!("now wrapper message is : {}", wrapper.data);
}
```

## Rust 中使用泛型类型的一些常见数据结构有哪些？

rust 中容器类型的结构和编码解码类的结构均为泛型结构，以支持更多的数据类型。 

比如 vector ,  hashmap , serde_json 等

## Rust 中有哪三种循环结构，它们如何使用？

三种循环 ：

1. loop 循环，适用于死循环 等同于while true 
2. for in 循环，适用于遍历某个数据结构，或者 执行一定次数
3. while  循环 给定 选择条件已终止循环， while true 等同于 loop

## 在 Rust 中，match、while let、let 和 if let 之间的区别是什么，每个应该在什么时候使用？

几种应用均可以用于多路选择结构，类似于其他语言里的swich 或者 多重if else 结构。区别在于 match 用户单条结构， while let 是模式匹配和 循环组合使用， if let  是和 选择结构组合使用， let  还可以用于变量结构声明。

## Rust 中有哪三种类型的注释？

```rust
/// 文档注释，一般位于文件开头
// 单行注释
/* */ 组合块状注释 
```