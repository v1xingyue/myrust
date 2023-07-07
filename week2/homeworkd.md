# homework

## rust 中不同的类型的变量，所有权是如何体现的？

固定类型，大小固定，这种类型一般存储在栈上。
可变类型，大小在运行的时候可以变化，一般保存在堆上。

赋值的时候，固定类型将发生 所有权的复制，可变类型则是发生所有权的转移。

## 你可以编写一个涉及生命周期参数的代码示例吗？

```rust
// 生命周期参数，只是用于标记函数参数。
// 并不是规定，换句话说，我实际调用的时候，可以不按照定义的生命周期长短来传递参数
// 生命周期参数： life(a) >= life(b)
pub fn hello_3<'a: 'b, 'b>(a: &'a u8, b: &'b u8) -> &'b u8 {
    b
}

```

## Rust 中的 trait 怎样理解，它有哪些功能

trait 类似于其他语言中的interface。用于定义符合一定规则的数据

应用场景

1. 泛型约束，可以限制传入数据的类型，这一点和其他语言里传递参数类似
2. trait object 可以把不同的数据统一为一种类型。比如 A 和 B 为两个不同的struct , 我如果想把他们放到一个vector 中，那么 我们可以使 A B 同时实现一个 空的trait T ， 然后 定义 类型为T 的类型即可。