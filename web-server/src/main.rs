use std::io::Read;
use std::net::{TcpListener, TcpStream};

#[derive(Debug)]
enum CustomErrorType {
    IncompleteMessageError,
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1234");
    match listener {
        Ok(valid_tcp) => {
            println!("Listening on 127.0.0.1:1234");
            for i in valid_tcp.incoming() {
                match i {
                    Ok(stream) => {
                        parse_tcp_stream(stream);
                    }
                    Err(er) => eprintln!("Error with incoming TCP stream: {er}"),
                }
            }
        }
        Err(er) => eprintln!("Error binding the port / setting up the listener: {er}"),
    }
}

fn parse_tcp_stream(mut stream_msg: TcpStream) {
    let mut buffer = [0; 1024];

    match stream_msg.read(&mut buffer) {
        Err(er) => println!("Error reading TCP stream into buffer: {er}"),
        Ok(bytes_read) => {
            let parsed_data = String::from_utf8_lossy(&buffer[..bytes_read]);

            if parsed_data.contains("\r\n\r\n") {
                println!("Message is complete");
                println!("Parsed request:\n{}", parsed_data);
            } else {
                println!("{:?}: Incomplete headers: missing CRLF CRLF separator: ", CustomErrorType::IncompleteMessageError);
            }
        }
    }
}
