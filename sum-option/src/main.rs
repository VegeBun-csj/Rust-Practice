fn sum(arr: &[u32]) -> Option<u32> {
    let mut res: u32 = 0;

    for i in arr.iter() {
        match res.checked_add(*i) {
            Some(x) => res = x,
            None => return None
        }
    }

    return Some(res)
}

fn main() {
    let arr = vec![1, 2, 3, 4, 5, 6,u32::MAX];
    let result = sum(&arr);
    // 判断函数返回的值是否是None，如果不是就打印出结果值，如果是就打印错误信息
    match result {
        None => println!("运算出现溢出"),
        Some(result) => println!("{}", result),
    }
}
