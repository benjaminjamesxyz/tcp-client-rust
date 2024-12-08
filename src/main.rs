use std::{
    io::{Read, Result, Write},
    net::TcpStream,
    time::Duration,
};

const HOST: &str = "127.0.0.1";
const PORT: &str = "4554";
const BUFFER_SIZE: usize = 512;
const TIMEOUT_SEC: u64 = 10;

fn main() -> Result<()> {
    let mut stream = match connect_to_server(&format!("{}:{}", HOST, PORT)) {
        Ok(stream) => stream,
        Err(error) => {
            println!("Error: {}", error);
            return Ok(());
        }
    };
    configure_connection(&mut stream)?;
    send_message(&mut stream, "Hello world..!")?;
    receive_message(&mut stream)?;
    Ok(())
}

fn connect_to_server(addr: &str) -> Result<TcpStream> {
    TcpStream::connect(addr)
}

fn configure_connection(stream: &mut TcpStream) -> Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(TIMEOUT_SEC)))
}

fn send_message(stream: &mut TcpStream, message: &str) -> Result<()> {
    stream.write_all(message.as_bytes())
}

fn receive_message(stream: &mut TcpStream) -> Result<String> {
    let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let bytes_read = stream.read(&mut buffer)?;
    Ok(String::from_utf8_lossy(&buffer[..bytes_read]).to_string())
}
