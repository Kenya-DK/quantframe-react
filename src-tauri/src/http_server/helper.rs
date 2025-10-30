use std::{io::Write, net::TcpStream};

pub fn respond_json(stream: &mut TcpStream, status: u16, body: &str) {
    let response = format!(
        "HTTP/1.1 {} OK\r\n\
         Content-Type: application/json\r\n\
         Content-Length: {}\r\n\
         Access-Control-Allow-Origin: *\r\n\
         Access-Control-Allow-Methods: GET, POST, PUT, DELETE, PATCH, OPTIONS\r\n\
         Access-Control-Allow-Headers: Content-Type, Authorization\r\n\
         \r\n{}",
        status,
        body.len(),
        body
    );
    let _ = stream.write_all(response.as_bytes());
}

pub fn respond_text(stream: &mut TcpStream, status: u16, msg: &str) {
    let response = format!(
        "HTTP/1.1 {} OK\r\n\
         Content-Type: text/plain\r\n\
         Content-Length: {}\r\n\
         Access-Control-Allow-Origin: *\r\n\
         Access-Control-Allow-Methods: GET, POST, PUT, DELETE, PATCH, OPTIONS\r\n\
         Access-Control-Allow-Headers: Content-Type, Authorization\r\n\
         \r\n{}",
        status,
        msg.len(),
        msg
    );
    let _ = stream.write_all(response.as_bytes());
}

pub fn respond_cors_preflight(stream: &mut TcpStream) {
    let response = "HTTP/1.1 200 OK\r\n\
         Access-Control-Allow-Origin: *\r\n\
         Access-Control-Allow-Methods: GET, POST, PUT, DELETE, PATCH, OPTIONS\r\n\
         Access-Control-Allow-Headers: Content-Type, Authorization\r\n\
         Access-Control-Max-Age: 86400\r\n\
         Content-Length: 0\r\n\
         \r\n";
    let _ = stream.write_all(response.as_bytes());
}
