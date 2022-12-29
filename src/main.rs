use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use rustwebserver::ThreadPool;

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool: ThreadPool = ThreadPool::new(100);

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();

        println!("Connection established!");

        pool.execute(|| {
            handle_connection(stream);
        });

    }
    println!("Shutting down.");
}


fn handle_connection(mut stream: TcpStream) {
    let buf_reader:BufReader<&TcpStream> = BufReader::new(&stream);

    let request_line:String = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line [..]{
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

        let contents:String = fs::read_to_string(filename).unwrap();
        let length:usize = contents.len();
    
        let response:String = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();

}
