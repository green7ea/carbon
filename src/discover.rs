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

impl Host
{
    pub fn new(user: String, ip: IpAddr) -> Self
    {
        Host {
            user,
            ip,
        }
    }
}

pub fn discover() -> std::io::Result<Vec<Host>>
{
    let socket = UdpSocket::bind("0.0.0.0:12346")?;
    let broadcast_addr = SocketAddrV4::new(MULTICAST_ADDR, 12345);

    let user = std::env::var("USER").unwrap_or(String::from("unknown"));
    let payload = serde_json::to_string(&Payload::new(&user))?;
    socket.send_to(payload.as_bytes(), broadcast_addr)?;

    // TODO we want to listen for a given time and list all the
    // results that came in during that time, not just take the first
    // result we get.

    let mut buf = [0; 1024];
    let (amt, src) = socket.recv_from(&mut buf)?;
    let buf = buf.get(0..amt).unwrap();

    let res: Payload = serde_json::from_slice(&buf).unwrap();
    Ok(vec![Host::new(res.user, src.ip())])
}
