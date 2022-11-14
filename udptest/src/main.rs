use std::env;
use std::net::UdpSocket;
use std::process;

fn do_server() {
    println!("Do server!\n");
    let socket = match UdpSocket::bind("127.0.0.1:44221") {
        Ok(socket) => socket,
        Err(error) => {
            println!("Can't bind socket: {:?}", error);
            process::exit(1);
        }
    };

    let mut buf = [0; 2000];
    let (recvlen, srcaddr) = socket.recv_from(&mut buf).unwrap();
    let buf = &mut buf[..recvlen];

    println!("== Server recved data==");
    hexdump::hexdump(buf);

    buf.reverse();
    socket.send_to(&buf, &srcaddr).unwrap();
}

fn do_client() {
    println!("Do client!\n");
    let socket = UdpSocket::bind("127.0.0.1:44222").unwrap();

    let mut buf = [0; 2000];

    let sendstr = "Hello world!";
    let sendlen = sendstr.len();
    
    let sbuf = &mut buf[..sendlen];
    sbuf.copy_from_slice(sendstr.as_bytes());
    socket.send_to(&buf[..sendlen], "127.0.0.1:44221").unwrap();

    let (recvlen, _srcaddr) = socket.recv_from(&mut buf).unwrap();
    let buf = &buf[..recvlen];

    println!("== Client recved data==");
    hexdump::hexdump(&buf);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("usage : cargo run -- server|client");
        process::exit(1);
    }

    if args[1].contains("s") {
        do_server();
    } else {
        do_client();
    }
}
