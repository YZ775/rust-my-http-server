use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::exit;

fn read_line(stream: &mut TcpStream) -> String {
    let mut buffer = [0; 1];
    let mut res = String::new();
    loop {
        stream.read(&mut buffer).unwrap();
        let c = buffer[0] as char;
        if c == '\n' {
            if res.chars().last().unwrap() == '\r' {
                res.pop();
                break;
            }
        }
        res.push(c);
    }
    res
}
fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080");
    if let Err(e) = listener {
        println!("Error: {}", e);
        exit(1);
    }
    let listener = listener.unwrap();
    let content = std::fs::read("./html/index.html").expect("file not found");
    loop {
        println!("accept...");
        let stream = listener.accept();
        if let Err(e) = stream {
            eprintln!("Error: {}", e);
            exit(1);
        }
        let stream = stream.unwrap();
        println!("connected to {}", stream.1);
        println!("-header-");
        let mut stream = stream.0;

        let first_line = read_line(&mut stream);
        let mut splitted_first_line = first_line.split_whitespace();
        let method = splitted_first_line.next().unwrap();
        println!("{} ", method);
        loop {
            let line = read_line(&mut stream);
            println!("{}", line);
            if line == String::from("") {
                break;
            }
        }
        let response_header_array: String =
            String::from("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n");
        let mut response_header_vec = response_header_array.into_bytes();
        response_header_vec.append(&mut content.clone());
        let response_header: &[u8] = &response_header_vec;
        let res = stream.write(response_header);
        if let Err(e) = res {
            eprintln!("faild to write. {}", e);
        }
        println!("---");
    }
}
