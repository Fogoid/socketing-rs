use std::{
    io::{Read, Result, Write, ErrorKind},
    net::TcpStream,
};

use clap::{Command, arg};

fn main() -> Result<()> {
    let cmd = Command::new("client")
        .args(&[
            arg!(<ADDRESS> "Address to connect to"),
            arg!(<MESSAGE> "Message to send")
        ]);
    
    let matches = cmd.get_matches();
    let address = matches.get_one::<String>("ADDRESS")
        .expect("Address is required");
    let msg = matches.get_one::<String>("MESSAGE")
        .expect("Message is required");

    let mut client = TcpStream::connect(address)?;
    let mut len_buf = [0; 8];

    let msg_size = msg.len().to_le_bytes();
    
    client.write(&msg_size)?;
    client.write_all(msg.as_bytes())?;

    client.read_exact(&mut len_buf)?;
    let len = usize::from_le_bytes(len_buf);

    let mut msg_buf = vec![0; len as usize];
    client.read_exact(&mut msg_buf)?;

    let msg = match String::from_utf8(msg_buf.to_vec()) {
        Ok(x) => x,
        Err(x) => return Err(std::io::Error::new(ErrorKind::Other, x)),
    };

    println!("Received response: {}", msg);

    Ok(())
}
