use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;
use std::sync::mpsc::*;
use uuid::Uuid;
use crate::socket_server_modules::Message;

#[allow(dead_code)]
pub struct Protocol {
    stream: TcpStream,
    uuid: Uuid,
    channel: Sender<Message::Message>
}

#[allow(dead_code)]
impl Protocol {
    
    pub fn new(stream: TcpStream, channel: Sender<Message::Message>) -> Protocol {

        let id = Uuid::new_v4();
        
        let _ = channel.send(Message::Message::CONNECT(stream.try_clone().unwrap(), id));

        Protocol {
            stream: stream,
            channel: channel,
            uuid: id,
        }
    }

    pub fn next_message(&mut self) -> Result<Vec<u8>, ()> {
        let mut number_buff: Vec<u8> = vec!();

        let mut buffered_reader = BufReader::new(&self.stream);
        
        let num_res = buffered_reader.read_until(b' ', &mut number_buff);

        if num_res.unwrap() == 0 {
            return Err(());
        }
        
        // println!("Length: {}", String::from_utf8_lossy(&number_buff));
        let length: usize = String::from_utf8_lossy(&number_buff).replace(" ", "").parse().unwrap();

        let mut message_buffer = vec![0u8; length];

        match buffered_reader.read_exact(&mut message_buffer) {
            Ok(_) => Ok(message_buffer),
            Err(_) => Err(())
        }
    }
      

    pub fn listen(&mut self) -> Result<(), ()>
    {   
        
        let res = self.next_message();       

        match res {
            Ok(msg) => {
                let _ = self.channel.send(Message::Message::MESSAGE(self.uuid.clone(),msg));
                Ok(())
            }
            Err(_) => {
                let _ = self.channel.send(Message::Message::DISCONNECT(self.uuid.clone()));
                Err(())
            }
        }
    }

}