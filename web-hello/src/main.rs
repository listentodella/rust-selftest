use std::{
    io::{prelude::*, BufReader},   //获取读写TcpStream流的特定trait
    net::{TcpListener, TcpStream}, //用于监听tcp连接
};

fn main() {
    //bind返回Result<T,E>,即可能成功或失败
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    //incoming返回一个迭代器,它提供了一系列的TcpStream类型的流
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        //println!("connection established!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    //这里增加了Vec<_>类型注解,表明希望将这些行收集到一个vector中
    let http_request: Vec<_> = buf_reader
        //遇到换行符就切分数据流,返回一个Result<String,std::io::Error>的迭代器
        .lines()
        //通过map的闭包,从Result<T,E>提取出String,但这里错误处理不太优雅
        .map(|line_result| line_result.unwrap())
        //浏览器通过连续发送两个换行符代表一个http请求的结束
        //take_while也接收一个闭包作为参数
        .take_while(|line| !line.is_empty())
        //收集所有非空行
        .collect();
    //println!("request:{:#?}", http_request);

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
}
