use std::{
    fs,                            //用于文件读写
    io::{BufReader, prelude::*},   //获取读写TcpStream流的特定trait
    net::{TcpListener, TcpStream}, //用于监听tcp连接
    thread,
    time::Duration,
};

use web_hello::ThreadPool;

fn main() {
    //bind返回Result<T,E>,即可能成功或失败
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);

    //incoming返回一个迭代器,它提供了一系列的TcpStream类型的流
    for stream in listener.incoming().take(2) {//这里只接收2个请求用于测试停机处理
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}
/*
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

    let status_line = "HTTP/1.1 200 OK\r\n\r\n";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContents-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
 */

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    //使用next从迭代器里获取第一项
    //第一个unwrap负责处理Option，并在迭代器没有项时停止程序
    //第二个unwrap负责处理Result
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            //访问 127.0.0.1:7878/sleep 即可触发
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContents-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
