use std::io::{self, prelude::*, BufReader, Write};
use std::net::TcpStream;
use std::str;
// TCP客户端
fn main() -> std::io::Result<()> {
    // 创建一个流，连接到server端
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    // 发10次请求
    for _ in 0..10 {
        // 创建一个输入
        let mut input = String::new();
        // 从标准输入读取
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");
        // 写入流
        stream
            .write(input.as_bytes())
            .expect("Failed to write to stream");
        
        // 从流中读取
        let mut reader = BufReader::new(&stream);
        // 创建一个buffer
        let mut buffer: Vec<u8> = Vec::new();
        // 一直读取，直到换行
        reader
            .read_until(b'\n', &mut buffer)
            .expect("Could not read into buffer");
        // 打印 
        println!("read from server: {}", 
            str::from_utf8(&buffer).expect("Could not write buffer as string"));
        println!("");
    }
    Ok(())
}
