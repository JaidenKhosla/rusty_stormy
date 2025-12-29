use std::collections::HashMap;
use std::io::Write;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex, mpsc::{self, *}};
use std::thread::{self};
use std::time::Duration;
use uuid::Uuid;

use crate::socket_server_modules::{Chunk::{Chunk, MAX_CONNECTIONS}, Message::Message, Protocol::Protocol};

type EVENT = fn(&mut TcpStream) -> ();
type MessageEvent = fn(&mut TcpStream, Vec<u8> ) -> ();


pub static LISTENING_SLEEP_TIME: u64 = 100;
pub static CHANNEL_SLEEP_TIME: u64 = 75;
pub struct SocketServer {
    socket: Arc<SocketAddr>,
    sender: Arc<Sender<Message>>,
    reciever: Arc<Mutex<Receiver<Message>>>,
    chunk_pool: Arc<Mutex<Vec<Chunk>>>,
    on_connect_event: Arc<Mutex<EVENT>>,
    on_message_event: Arc<Mutex<MessageEvent>>,
    on_disconnect_event: Arc<Mutex<EVENT>>,

    is_shutting_down: Arc<Mutex<bool>>
}

#[allow(dead_code)]
impl SocketServer {
    pub fn new(socket: &str) -> SocketServer {
        
        let ( sender, reciever ) = mpsc::channel();
        let socket_address = Arc::new(socket.parse().expect("Must be a valid socket address"));
            
        let mut init_chunk_pool = vec!();
        
        //init is 2 chunks
        init_chunk_pool.push(Chunk::new());
        init_chunk_pool.push(Chunk::new());

        SocketServer {
            socket: socket_address,
            sender: Arc::new(sender),
            reciever: Arc::new(Mutex::new(reciever)),
            chunk_pool: Arc::new(Mutex::new(init_chunk_pool)),
            on_connect_event: Arc::new(Mutex::new(def_on_connect)),
            on_message_event: Arc::new(Mutex::new(def_on_message)),
            on_disconnect_event: Arc::new(Mutex::new(def_on_disconnect)),
            is_shutting_down: Arc::new(Mutex::new(false))
        }
    }

    pub fn run(&mut self){

        
        *self.is_shutting_down.lock().unwrap() = false;
        
        let sender = Arc::clone(&self.sender);
        let arc_reciever = Arc::clone(&self.reciever);
        let chunk_pool: Arc<Mutex<Vec<Chunk>>> = Arc::clone(&self.chunk_pool);
        let socket_addr = Arc::clone(&self.socket);
        
        let on_connect_event = Arc::clone(&self.on_connect_event);
        let on_message_event = Arc::clone(&self.on_message_event);
        let on_disconnect_event = Arc::clone(&self.on_disconnect_event);
        
        let is_shutting_down_1 = Arc::clone(&self.is_shutting_down);
        let is_shutting_down_2 = Arc::clone(&self.is_shutting_down);
        
        let listener_thread = thread::spawn(move || {
            
            let listener = TcpListener::bind(&*socket_addr).unwrap();
            
            let mut chunk_index = 0usize;
            
            while !is_shutting_down_1.lock().unwrap().clone() {
                
                let (socket, _) = listener.accept().unwrap();
                
                let protcol_handler = Protocol::new(socket, (*sender).clone());
                
                
                let mut length = chunk_pool.lock().unwrap().len();
                
                if length == 0 {
                    
                    let mut locked_chunk_pool = chunk_pool.lock().unwrap();
                    locked_chunk_pool.push(Chunk::new());
                    locked_chunk_pool.push(Chunk::new());

                    length = 2;
                }
                
                let selected_chunk = &mut chunk_pool.lock().unwrap()[chunk_index];
                
                
                if chunk_index == length-1 && selected_chunk.size() >= MAX_CONNECTIONS {
                    chunk_pool.lock().unwrap().push(Chunk::new());
                    length += 1;
                }
                
                chunk_index = (chunk_index+1)%length;
                
                selected_chunk.append(protcol_handler);
                
                if !selected_chunk.already_running() {
                    selected_chunk.run();
                }

                thread::sleep(Duration::from_millis(LISTENING_SLEEP_TIME));
            }
        });
        
        
        let _channel_thread = thread::spawn(move || {
            let mut map: HashMap<Uuid, TcpStream> = HashMap::new();
            let reciever = arc_reciever.lock().unwrap();
            
            
            while !is_shutting_down_2.lock().unwrap().clone() {
                let incoming_message = reciever.recv().unwrap();
                
                match incoming_message {
                    Message::CONNECT(stream, id) => {
                        map.insert(id, stream);
                        
                        let borrowed_stream=  map.get_mut(&id).unwrap();
                        
                        on_connect_event.lock().unwrap()(borrowed_stream);
                    },
                    
                    Message::MESSAGE(id, bytes) => {
                        let stream = map.get_mut(&id).unwrap();
                        
                        on_message_event.lock().unwrap()(stream, bytes);
                    },
                    
                    Message::DISCONNECT(id) => {
                        
                        let stream = map.get_mut(&id).unwrap();
                        
                        on_disconnect_event.lock().unwrap()(stream);
                        
                        map.remove(&id);
                    },
                    
                    Message::BROADCAST( bytes) => {
                        for stream_id in map.keys() {
                            let _ = map.get(stream_id).unwrap().write(&bytes);
                        }
                    }
                }

                thread::sleep(Duration::from_millis(CHANNEL_SLEEP_TIME));
            }
            
        });
        
        println!("Running on {}", self.socket);

        let _ = listener_thread.join();
    }
    
    pub fn broadcast(&mut self, bytes: Vec<u8>){
        self.sender.send(Message::BROADCAST(bytes)).unwrap()
    }
    
    pub fn shutdown(&mut self){

        println!("Shutting down on {}", self.socket);
        
        *self.is_shutting_down.lock().unwrap() = true;
        
        let mut chunk_pool = self.chunk_pool.lock().unwrap();
        
        chunk_pool.iter_mut().for_each(|chunk| chunk.shutdown());
        chunk_pool.clear();
        
    }
}

fn def_on_connect(stream: &mut TcpStream) {
    println!("{} connected.", stream.peer_addr().unwrap());
}

fn def_on_message(stream: &mut TcpStream, message: Vec<u8>){
    println!("{} sent {}", stream.peer_addr().unwrap(), String::from_utf8_lossy(&message));
}

fn def_on_disconnect(stream: &mut TcpStream) {
    println!("{} disconnected.", stream.peer_addr().unwrap());
}
