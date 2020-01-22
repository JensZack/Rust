#![allow(dead_code)]

use std::{str, env, io};
use std::net::{UdpSocket, SocketAddrV4, Ipv4Addr};

fn ip_to_string(input: &[u8]) -> String {
    let mut output = String::from("");
    for num in input.iter() {
        output += &num.to_string();
        output.push('.');
    }
    output.pop();
    return output
}

/*
fn send_data(data: &str, ip: &[u8; 4], port: &u16) -> std::io::Result<()> {
    {
        let socket_addr = String::from(ip_to_string(ip)) + ":" + &port.to_string();
        let socket = UdpSocket::bind(socket_addr)?;

        let buf = data.as_bytes();
        // socket.send_to(buf, socket_addr).expect("couldn't send data");
    }
    Ok(())
}

fn receive_data(ip: &[u8; 4], port: &u16) -> std::io::Result<()> {

    let mut buf = [0; 10];

    // let ip = Ipv4Addr::new(ip[0], ip[1], ip[2], ip[3]);
    // let socket = SocketAddrV4::new(ip, *port);

    let socket = String::from(ip_to_string(ip)) + ":" + &(*port).to_string();
    let socket = UdpSocket::bind(socket)?;

    let (number_of_bytes, src_addr) = socket.recv_from(&mut buf)
        .expect("didn't receive any data");
    let filled_buf = &mut buf[..number_of_bytes];

    let message: &str = str::from_utf8(filled_buf).expect("no message");
    println!("The message received is: {}", message);
    Ok(())
}
*/

fn send(socket: &UdpSocket, msg: &str) -> usize {
    println!("Sending Message");
    let msg = msg.as_bytes();
    socket.connect("127.0.0.1:8080");
    let result = socket.send_to(msg, socket)
        .expect("failed to send message");
    result
}

fn listen(socket: &UdpSocket, mut buffer: &mut [u8]) -> usize {

    let (num_bytes, src_addr) = socket.recv_from(&mut buffer)
        .expect("nothing received");

    println!("number of bytes received: {:?}, from {:?}", num_bytes, src_addr);
    num_bytes
}

fn init_socket(ip: &[u8; 4], port: &u16) -> UdpSocket {
    println!("initializing socket");
    let socket = String::from(ip_to_string(ip) + ":" + &(*port).to_string());
    let socket = UdpSocket::bind(socket)
        .expect("failed to bind socket");
    socket
}

fn main() {

    // listen works when sending udp using nc -u localhost 8080
    // send does not work. I don't think I understand how sockets work yet
    let nodetype = env::args().nth(1).unwrap();
    println!("the first argument is {}", nodetype);

    let data = "dogs and cats";
    let address: [u8; 4] = [127,0,0,1];
    let port: u16 = 8080;

    let socket = init_socket(&address, &port);

    if nodetype == String::from("send") {
        send(&socket, data);
    }

    let mut buffer: [u8; 100] = [0; 100];
    let msg_response: usize = 0;
    if nodetype == String::from("listen") {
        while msg_response == 0 {
            let msg_response = listen(&socket, &mut buffer);
        }
        let message: &str = str::from_utf8(&buffer).expect("decoding error");
        println!("Message Received: {}", &message[..msg_response]);
    }
}

