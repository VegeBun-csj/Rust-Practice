- Rust 贴心地提供了派生宏（derive macro），可以大大简化一些标准接口的定义，比如  `#[derive(Debug)]`为数据结构实现了debug trait，提供了 debug 能力，这样可以通过 {:?}，用 println! 打印出来。
- Clone 让数据结构可以被复制，而 Copy 则让数据结构可以在参数传递的时候自动按字节拷贝
- Rust 的 for 循环可以用于任何实现了`IntoIterator trait` 的数据结构,IntoIterator 会生成一个迭代器，for 循环不断从迭代器中取值，直到迭代器返回 None 为止。因而，for 循环实际上只是一个语法糖，编译器会将其展开使用 loop 循环对迭代器进行循环访问，直至返回 None
- Rust把错误封装在`Result<T, E>` 类型中，同时提供了 `?` 操作符来传播错误，方便开发。Result<T, E> 类型是一个泛型数据结构，`T` 代表成功执行返回的结果类型，`E` 代表错误类型
- 