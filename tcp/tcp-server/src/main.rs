use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;


// 处理客户端的流数据
fn handle_client(mut stream: TcpStream) -> Result<(), Error>{
    // 创建一个buffer数据
    let mut buf = [0; 512];
    for _ in 0..1000 {
        // 读取流数据
        let bytes_read = stream.read(&mut buf)?;
        // 判断是否结束
        if bytes_read == 0 {
            return Ok(());
        }

        // 把数据写回去
        stream.write(&buf[..bytes_read])?;
        // 睡眠一秒钟
        thread::sleep(time::Duration::from_secs(1 as u64));
    }

    Ok(())
}


fn main() -> std::io::Result<()> {
    // 创建一个服务器的listener
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // 创建一个容器来存放线程返回的句柄
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();

    // 获取listener传来的内容
    for stream in listener.incoming() {
        // 进行模式匹配
        match stream{
            // 如果没有流数据，就打印错误
            Err(e) => println!("{}",e),
            // 如果有流数据就进行处理
            Ok(stream) => {
                // 创建一个线程来处理流数据，参数是一个闭包
                let handle = thread::spawn(move || {
                    // 调用handle_client函数来处理客户端的流数据
                    handle_client(stream)
                    .unwrap_or_else(|error| eprintln!("{:?}", error));
                });
                //存放线程句柄
                thread_vec.push(handle);
            }

        }
    }

    //等待线程结束
    for handle in thread_vec {
        handle.join().unwrap();
    }

    Ok(())
}
