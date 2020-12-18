use std::net::{TcpListener, TcpStream};
use ftp_utils::{ftp_read, ftp_write};

fn server(ip: &str) {
    let listener = TcpListener::bind(ip).unwrap();

    println!("[INFO] Listenng on {}", ip);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("\t[INFO] {} Connected", stream.peer_addr().unwrap());

                match ftp_write::stream_file(&mut stream, "test.txt") {
                    Ok(()) => {
                        println!("\t[INFO] {} Downloaded File", stream.peer_addr().unwrap());
                    },
                    Err(_) => {
                        println!("\t[ERROR] {} Cannot Download File", stream.peer_addr().unwrap());
                    }
                };
            },
            Err(_) => {
                println!("[ERROR] Cannot Connect");
            }
        }
    }
}

fn client(ip: &str) {
    let mut stream = TcpStream::connect(ip).unwrap();

    match ftp_read::stream_to_file(&mut stream, "newtest.txt") {
        Ok(()) => {
            println!("[INFO] Downloaded File");
        },
        Err(_) => {
            println!("[ERROR] Cannot Download File");
        }
    };
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args[1] == "server".to_string() {
        server("localhost:8888");
    } else if args[1] == "client".to_string() {
        client("localhost:8888");
    } else {
        println!("[ERROR] Invalid Argument");
    }
}
