use std::{
    io::{ErrorKind, Read, Result, Write},
    net::TcpListener,
    vec
};

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let mut len_buf = [0; 8];

    for stream in listener.incoming() {
        let mut stream = stream?;
        stream.read_exact(&mut len_buf)?;

        let len = usize::from_le_bytes(len_buf);
        let mut msg_buf = vec![0; len as usize];
        stream.read_exact(&mut msg_buf)?;

        let msg = match String::from_utf8(msg_buf.to_vec()) {
            Ok(x) => x,
            Err(x) => return Err(std::io::Error::new(ErrorKind::Other, x)),
        };

        println!("Message received: {}", msg);
        let response = format!("Message successfully read. Bytes read: {}", len);
        let res_bytes = response.len().to_le_bytes();

        stream.write_all(&res_bytes)?;
        stream.write_all(response.as_bytes())?;
    }

    Ok(())
}
