# week3

## trait object （复习week2）

1. 使用 Vec<&dyn TraitName> 来定义一组trait ,然后完成批量调用
2. trait 中定义的方法均为pub 方法，无需独立声明
3. Vec 中传递临时变量的引用，存在一定bug，故rustc 不建议这么使用，解决办法有两种
   a 使用变量定义接受struct 的值
   b 使用Box 封装引用，取代临时变量，接管生命周期
   参考代码: [callers.rs](./src/callers.rs)
4. rust 内部trait
   参看 [app.rs](./src/app.rs)
   1. From trait (初始化数据)
   2. Into trait （转换数据）
   3. AsRef ,AsMut （获取引用，和可变应用）可以将内部的数据导出。

## 闭包

1. 属于匿名函数，可以接受参数，接收引用。调用时传递当前的环境变量。
2. 闭包 分类有三种 ： 无参数，捕获但不修改变量, 捕获修改变量
3. 闭包相关的 trait 有 Fn FnOnce FnMut 。调用的时候，才会实现对应的trait
4. 偷懒的话，就不用管 闭包的类型，尽管用就行。系统会帮你实现

## 迭代器

1. vec map 等集合 均实现了 Iterator 和 IntoIterator 两个trait
2. 有了 这个迭代器，就可以进行循环了
3. 同理，如果你想定义一个支持for 循环的结构，你也需要实现 Iterator
4. 迭代器惰性，不触发，不执行
5. 迭代器可以和闭包结合使用，万策划给你复杂的功能

一个比较有意思的例子,当 闭包返回的数据，不足以破坏内部的结构的时候，即使他是FnOnce ， 依然可以被调用多次

```rust
let call_once = {
   let x = 3;
   let s = String::from("hello");
   move || {
      println!("x is {}", x);
      println!("string is : {}", s);
      x
   }
};
call_once();
call_once();
```

## 智能指针

1. String , Vec, Box 自动实现了某些功能的类型 比如 copy drop 这些
2. 自己实现一个 Drop trait (有点类似面向对象里的析构函数)