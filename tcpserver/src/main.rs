use std::net::TcpListener;
use std::io::{Read,Write};

fn main() {
    let listener=TcpListener::bind("127.0.0:3000").unwrap();


    println!("Hello, world!");

    // let result=listener.accept().unwrap();

    for stream in listener.incoming(){
       let mut stream=stream.unwrap();
        let mut buffer=[0;1024];
        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }
}
