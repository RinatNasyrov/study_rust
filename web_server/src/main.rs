use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7879").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "web_server/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "web_server/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    // Хз почему при указании "Content-Length: {length}" после status_line
    // браузер выводил ответ текстом, а не рендерил страницу
    // let length = contents.len();

    let response = format!("{status_line}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
