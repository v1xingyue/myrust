# week3

## trait object

1. 使用 Vec<&dyn TraitName> 来定义一组trait ,然后完成批量调用
2. trait 中定义的方法均为pub 方法，无需独立声明
3. Vec 中传递临时变量的引用，存在一定bug，故rustc 不建议这么使用，解决办法有两种
   a 使用变量定义接受struct 的值
   b 使用Box 封装引用，取代临时变量，接管生命周期
   参考代码: [main.rs](./src/main.rs)