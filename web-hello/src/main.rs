use std::net::TcpListener; //用于监听tcp连接

fn main() {
    //bind返回Result<T,E>,即可能成功或失败
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    //incoming返回一个迭代器,它提供了一系列的TcpStream类型的流
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("connection established!");
    }
}
