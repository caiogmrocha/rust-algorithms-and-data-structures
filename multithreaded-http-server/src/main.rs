use std::{io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    for stream_result in listener.incoming() {
        let stream = stream_result.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let response = mux(http_request);

    stream.write_all(response.as_bytes()).unwrap();
}

fn mux(http_request: Vec<String>) -> String {
    match http_request[0].as_str() {
        "GET / HTTP/1.1" => {
            let status_line = "HTTP/1.1 200 OK";
            let contents_section = "Hello World";
            let contents_length = contents_section.len();

            let response = format!("{status_line}\r\nContent-Length: {contents_length}\r\n\r\n{contents_section}");

            response
        },
        _ => {
            let status_line = "HTTP/1.1 404 NOT FOUND";
            let contents_section = "Not Found";
            let contents_length = contents_section.len();

            let response = format!("{status_line}\r\nContent-Length: {contents_length}\r\n\r\n{contents_section}");

            response
        }
    }
}