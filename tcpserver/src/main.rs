// 引入作用域, 以便代码能使用作用域, 避免作用域未声明
// use std::io::prelude::*;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};


fn handle_connection(mut stream: TcpStream) {
    // 语法说明地址: https://www.twle.cn/c/yufei/rust/rust-basic-array.html
    // 数组声明语法: let variable_name:[dataType;size] = [default_value_for_elements,size];
    // 声明数组: 数组元素初始化为0, 数组长度1024字节. 编译器推断数组数据类型为i32. (数字类型默认是 i32)
    // mut 用于声明数组元素可修改.
    let mut buffer = [0; 1024];
    // 从socket接收, 保存到数组
    stream.read(&mut buffer).unwrap();
    // 函数from_utf8_lossy: Converts a slice of bytes to a string, including invalid characters
    // &buffer[start_index..end_index]用于数组切片. 省略区间的第一个数字默认是零，省略区间第二个数字默认是数组的长度
    println!("Receive: {}", String::from_utf8_lossy(&buffer[..]));
    // 把数组中之前接收到的内容写入socket缓冲区
    stream.write(&buffer).unwrap();
    // 刷新socket缓冲区
    stream.flush().unwrap();
}


fn main() {
    let host = "0.0.0.0";
    let port = 8080;

    // unwrap() 表示bind函数错误时终止程序并执行panic
    // 指定服务监听ip和port
    let listener = TcpListener::bind(format!("{host}:{port}")).unwrap();
    println!("Listening at {host}:{port}");

    // 接受连接
    for stream in listener.incoming() {
        // 标准的错误处理（模式匹配）
        match stream {
            Ok(stream) => {
                // 处理socket
                handle_connection(stream);
            }
            Err(e) => {
                panic!("incomming() error: {}", e)
            }
        }
    }
}
