use std::net::{TcpListener, TcpStream};

pub struct StreamIter {
    ip: String,
    is_server: bool,
    listener: Option<TcpListener>,
}

impl Iterator for StreamIter {

    type Item = TcpStream;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_server {
            let mut incoming = match self.listener.as_mut() {
                Some(listener) => listener.incoming(),
                None => {
                    self.listener = Some(match TcpListener::bind(self.ip.as_str()) {
                        Ok(listener) => listener,
                        Err(_) => panic!("Unable to bind to {}", self.ip)
                    });
                    self.listener.as_mut().unwrap().incoming()
                }
            };
            println!("Waiting for connection from client");
            match incoming.next() {
                Some(stream) => match stream {
                        Ok(client) => {
                            println!("Connection established");
                            Some(client)
                        },
                        Err(_) => panic!("Unable to get stream")
                    },
                None => None
            }
        } else {
            println!("Connecting to server");
            match TcpStream::connect(self.ip.as_str()) {
                Ok(client) => {
                    println!("Connection established");
                    Some(client)
                },
                Err(_) => panic!("Unable to connect to {}:", self.ip)
            }
        } 
    }
}

pub fn get_streams(is_server: bool, ip: String) -> StreamIter {
    StreamIter {
        ip,
        is_server,
        listener: None,
    }
}