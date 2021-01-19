use std::net::{TcpStream};
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6666")?;
    stream.set_nonblocking(true)?;
    let mut buf;
    loop {
        buf = String::new();
        std::io::stdin().read_line(&mut buf)?;
        let msg = buf.trim().to_string();
        if msg == ":quit" { break }
        let mut bytes = msg.clone().into_bytes();
        bytes.resize(16, 0);
        stream.write_all(&bytes)?;
    }
    Ok(())
}
