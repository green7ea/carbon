mod discover;

use discover::Payload;
use std::{
    io::{Read, Write},
    net::{Ipv4Addr, Shutdown, TcpListener, TcpStream, UdpSocket},
};

pub fn handle_client(mut stream: TcpStream)
{
    let mut header = [0 as u8; 2];
    stream.read_exact(&mut header).unwrap();

    let size = (header[0] as u16) | ((header[1] as u16) << 8);

    let mut filename = Vec::new();
    filename.resize(size as usize, 0);
    stream.read_exact(filename.as_mut_slice()).unwrap();
    let filename = std::str::from_utf8(&filename).unwrap();

    println!("Receiving {}", filename);
    let mut file = std::fs::File::create(&filename).unwrap();

    let mut data = vec![0 as u8; 4096];

    loop
    {
        let size = stream.read(data.as_mut_slice()).unwrap();
        if size <= 0
        {
            break;
        }

        file.write(&data[0..size]).unwrap();
    }

    stream.shutdown(Shutdown::Both).unwrap();
}

pub fn receive() -> std::io::Result<()>
{
    let user = std::env::var("USER").unwrap();
    let socket = UdpSocket::bind("0.0.0.0:12345")?;

    socket.join_multicast_v4(
        &discover::MULTICAST_ADDR,
        &Ipv4Addr::new(0, 0, 0, 0),
    )?;

    let mut buf = [0; 1024];

    loop
    {
        let (amt, src) = socket.recv_from(&mut buf)?;
        let buf = buf.get(0..amt).unwrap();

        let res: Payload = serde_json::from_slice(&buf).unwrap();

        println!("Question from {}@{}", res.user, src.ip());

        let payload = serde_json::to_string(&Payload::new(&user)).unwrap();
        socket.send_to(payload.as_bytes(), &src)?;
    }

    /* TODO where do we do this?
    socket.leave_multicast_v4(
        &discover::MULTICAST_ADDR,
        &Ipv4Addr::new(0, 0, 0, 0),
    )?;
    */
}

pub fn listen_for_files()
{
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();

    for stream in listener.incoming()
    {
        match stream
        {
            Ok(stream) =>
            {
                println!("New connection: {}", stream.peer_addr().unwrap());
                std::thread::spawn(move || handle_client(stream));
            },
            Err(e) =>
            {
                println!("Error: {}", e);
            },
        }
    }

    drop(listener);
}

fn main() -> std::io::Result<()>
{
    std::thread::spawn(listen_for_files);
    receive()
}
