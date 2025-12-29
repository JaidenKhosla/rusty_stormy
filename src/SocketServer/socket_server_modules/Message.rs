use std::net::TcpStream;
use uuid::*;

#[allow(dead_code)]
pub enum Message {
    CONNECT(TcpStream, Uuid),
    MESSAGE(Uuid, Vec<u8>),
    BROADCAST(Vec<u8>),
    DISCONNECT(Uuid)
}

