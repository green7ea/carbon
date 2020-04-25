mod discover;

use discover::discover;
use std::{
    io::{Read, Write},
    net::TcpStream,
};

fn main()
{
    let mut args = std::env::args().skip(1);
    let filename = args
        .next()
        .expect("Please provide a filename as a first argument");
    let user = args
        .next()
        .expect("Please provide a user as a second argument");
    let mut file = std::fs::File::open(&filename).expect("File not found");
    let filename = std::path::Path::new(&filename)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    let hosts = match discover()
    {
        Ok(list) => list,
        Err(error) => panic!("Ran into a problem: {}", error),
    };
    let ip = hosts
        .filter(|host| host.user == user)
        .nth(0)
        .expect("User not found")
        .ip;

    let mut tcp = TcpStream::connect(format!("{}:3333", ip)).unwrap();
    println!("Sending {} to {}", filename, ip);

    let header = filename.as_bytes();
    let header_len = header.len() as u16;

    let header_len: [u8; 2] = [
        (header_len & 0xFF) as u8,
        ((header_len & 0xFF00) >> 8) as u8,
    ];

    tcp.write(&header_len).unwrap();
    tcp.write(header).unwrap();

    let mut buffer = vec![0 as u8; 4096];

    loop
    {
        let size = file.read(&mut buffer).unwrap();
        if size <= 0
        {
            break;
        }

        tcp.write(&buffer[0..size]).unwrap();
    }
    println!("{} sent", filename);
}
