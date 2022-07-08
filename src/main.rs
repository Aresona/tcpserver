use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    // 定义指定大小的 buffer, 用来接收 message
    let mut buf = [0; 10];
    // 循环接收信息
    loop {
        // 模式匹配 + 读取流内容
        match stream.read(&mut buf) {
            Ok(size) => {
                // 将读取到的流内容再回写
                stream.write(&buf[0..size]).unwrap();
            }
            Err(_) => {
                println!("读取流内容失败")
            }
        }
    }
}

fn main() {
    // 服务监听
    let listener = TcpListener::bind("0.0.0.0:8888").unwrap();
    // 等待连接
    for stream in listener.incoming() {
        // 模式匹配
        println!("new connection");
        // 创建新线程来处理新请求
        thread::spawn(move || handle_client(stream.unwrap()));
    }
}
