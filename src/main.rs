use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                handle_connection(&mut stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(stream: &mut TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    let lines: Vec<&str> = request.lines().collect();
    let first_line = *lines.first().unwrap();
    let first_line_only_words: Vec<&str> = first_line.split_whitespace().collect();

    println!("Request: {}", request);

    let response = if first_line.starts_with("GET / HTTP/1.1") {
        "HTTP/1.1 200 OK\r\n\r\n".to_string()
    } else if first_line.starts_with("GET /echo/") {
        let to_echo = first_line_only_words.get(1).unwrap().split_at(6).1;
        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            to_echo.len(),
            to_echo,
        )
    } else {
        "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
    };

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
