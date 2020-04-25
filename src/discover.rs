use serde::{Deserialize, Serialize};

use std::net::{IpAddr, Ipv4Addr, SocketAddrV4, UdpSocket};

pub const MULTICAST_ADDR: std::net::Ipv4Addr = Ipv4Addr::new(224, 0, 0, 231);

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload
{
    pub user: String,
}

impl Payload
{
    pub fn new(hostname: &str) -> Self
    {
        Payload {
            user: String::from(hostname),
        }
    }
}

pub struct Host
{
    pub user: String,
    pub ip: IpAddr,
}

impl Host {
    pub fn to_string(&self) -> String {
        format!("{}@{}", self.user, self.ip)
    }
}

pub struct HostIterator
{
    socket: UdpSocket,
    buffer: Vec<u8>,
}

impl Iterator for HostIterator
{
    type Item = Host;

    fn next(&mut self) -> Option<Self::Item>
    {
        let (amt, src) = match self.socket.recv_from(&mut self.buffer)
        {
            Ok(payload) => payload,
            Err(_) => return None,
        };
        let buf = self.buffer.get(0..amt).unwrap();

        let res: Payload = match serde_json::from_slice(&buf)
        {
            Ok(payload) => payload,
            Err(_) => return None,
        };

        return Some(Host {
            user: res.user,
            ip: src.ip(),
        });
    }
}

pub fn discover() -> std::io::Result<HostIterator>
{
    let socket = UdpSocket::bind("0.0.0.0:12346")?;
    let broadcast_addr = SocketAddrV4::new(MULTICAST_ADDR, 12345);
    socket.set_read_timeout(Some(std::time::Duration::from_secs(1)))?;

    let user = std::env::var("USER").unwrap_or(String::from("unknown"));
    let payload = serde_json::to_string(&Payload::new(&user))?;
    socket.send_to(payload.as_bytes(), broadcast_addr)?;

    let buffer = vec![0; 4096];
    Ok(HostIterator {
        socket,
        buffer,
    })
}
