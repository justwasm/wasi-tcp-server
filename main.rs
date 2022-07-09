use std::io::{Read, Write};
use std::net::SocketAddr;
use std::str::from_utf8;
use wasmedge_wasi_socket::{TcpListener, TcpStream};

fn handle_client((mut stream, addr): (TcpStream, SocketAddr)) -> std::io::Result<()> {
    // 打印客户端 IP 和端口
    let local_addr = stream.local_addr()?;
    println!("{} <-> {}", addr.to_string(), local_addr);

    // 循环
    loop {
        // 初始化缓冲区
        let mut buffer = [0; 1024];
        // 读取数据
        stream.read(&mut buffer)?;
        // 转换为字符串
        let line = from_utf8(&buffer).unwrap();
        // 打印
        print!("get message: {}", line);
        println!("sendback reversed message...");
        // 字符串转换为字节切片返回给客户端
        stream.write(&line.as_bytes())?;
    }
}

fn main() -> std::io::Result<()> {
    // 从 PORT 环境变量获取端口, 默认 1234
    let port = std::env::var("PORT").unwrap_or("1234".to_string());
    println!("listening at 127.0.0.1:{}", port);
    // 监听端口并使用标准方式处理错误
    let listener = match TcpListener::bind(format!("127.0.0.1:{}", port), false) {
        Ok(ln) => ln,
        Err(e) => return Err(e),
    };
    // 处理来自客户端的请求, 同一时间只能处理一个连接
    loop {
        handle_client(listener.accept(false).unwrap())?;
    }
}
