fn main() {
    // 迭代器的使用
    // let v = vec!["a".to_string(), "b".to_string()];
    // // into_iter得到的是一个迭代器
    // for s in v.into_iter() {
    //     // s has type String, not &String
    //     println!("{}", s);
    // }



    // resolve Result<T,E>
    /* 
    错误处理
     */
    let twenty = double_number("10");
    print(twenty);
    // 下面提供了更加有用的错误消息。
    let tt = double_number_map("t");
    print(tt);
}

use std::num::ParseIntError;
// 返回类型重写之后，我们使用模式匹配，而不使用 `unwrap()`。
fn double_number(number_str: &str) -> Result<i32, ParseIntError> {
    match number_str.parse::<i32>() {
        Ok(n) => Ok(2 * n),
        Err(e) => Err(e),
    }
}
// 就像 `Option`，我们可以使用组合算子，如 `map()`。
// 此函数在其他方面和上述的示例一样，并表示：
// 若值有效则修改 n，否则传递错误。
fn double_number_map(number_str: &str) -> Result<i32, ParseIntError> {
    number_str.parse::<i32>().map(|n| 2 * n)
}
fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}
