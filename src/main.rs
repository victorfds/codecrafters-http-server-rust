use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread::spawn,
};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                spawn(|| handle_connection(stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    let lines: Vec<&str> = request.lines().collect();
    let first_line = *lines.first().unwrap();
    let third_line = *lines.get(2).unwrap();

    println!("Request: {}", request);

    let response = match first_line {
        line if line.starts_with("GET / HTTP/1.1") => "HTTP/1.1 200 OK\r\n\r\n".to_string(),
        line if line.starts_with("GET /echo/") => {
            let to_echo = line.split_whitespace().nth(1).unwrap().split_at(6).1;
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                to_echo.len(),
                to_echo,
            )
        }
        line if line.starts_with("GET /user-agent") => {
            let third_line_only_words: Vec<&str> = third_line.split_whitespace().collect();
            let user_agent_value = *third_line_only_words.get(1).unwrap();

            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                user_agent_value.len(),
                user_agent_value
            )
        }
        _ => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
    };

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
