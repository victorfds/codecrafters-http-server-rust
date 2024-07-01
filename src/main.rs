use std::{
    env,
    fs::{self, File},
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread::spawn,
};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
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
        line if line.starts_with("GET /file") => {
            let file_name = line.split_whitespace().nth(1).unwrap().split_at(7).1;
            let env_args: Vec<String> = env::args().collect();
            let mut dir = env_args[2].clone();
            dir.push_str(file_name);
            let file = fs::read(dir);

            if let Ok(response) = file {
                format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}", response.len(), String::from_utf8(response).expect("File content error"))
            } else {
                "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
            }
        }
        line if line.starts_with("POST /files/") => {
            let file_name = line.split_whitespace().nth(1).unwrap().split_at(9).1;
            let env_args: Vec<String> = env::args().collect();
            let mut dir = env_args[2].clone();
            dir.push_str(file_name);
            let mut file = File::create(dir).unwrap();
            let contents = lines[5].trim_end_matches('\0');
            file.write_all(contents.as_bytes()).unwrap();
            "HTTP/1.1 201 Created\r\n\r\n".to_string()
        }
        _ => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
    };

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
