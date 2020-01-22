use std::net::{UdpSocket, SocketAddr};

enum IpAddr {
    V4([u8; 4]),
    V6(String),
}

fn send_data(data: &str, address: &IpAddr, port: &str) -> std::io::Result<()> {
    let Ip = match address { IpAddr::V4(Ip) => Ip, IpAddr::V6(&Ip) => Ip, };

    {
        let socket = String::from(Ip) + ":" + port;
        let socket = UdpSocket::bind(socket)?;

        let addr = SocketAddr::from((ip, port));
    }
    Ok(())
}


fn main() {
    let data = "dogs and cats";
    let address = IpAddr::V4([127, 0, 0, 1]);
    let port = "22";
    send_data(&data, &address, &port);
}
