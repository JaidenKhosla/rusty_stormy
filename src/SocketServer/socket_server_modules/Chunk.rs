use std::sync::{Mutex, Arc};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use crate::socket_server_modules::Protocol::Protocol;

#[allow(dead_code)]
pub static MAX_CONNECTIONS: usize = 10; 

#[allow(dead_code)]
//In milliseconds
pub static SLEEP_TIME: u64 = 50;

#[allow(dead_code)]
pub struct Chunk {
    connections: Arc<Mutex<Vec<Protocol>>>,
    already_running: bool,
    shutdown: Arc<Mutex<bool>>
}

#[allow(dead_code)]
impl Chunk {

    pub fn new() -> Chunk {
        Chunk {
            connections: Arc::new(Mutex::new(vec!())),
            shutdown: Arc::new(Mutex::new(false)),
            already_running: false
        }
    }

    pub fn append(&mut self, protocol: Protocol){
        self.connections.lock().unwrap().push(protocol);

    }

    pub fn size(&self) -> usize {
        self.connections.lock().unwrap().len()
    }

    pub fn already_running(&self) -> bool {
        self.already_running
    }

    pub fn run(&mut self) -> JoinHandle<()>{

        self.already_running = true;

        let arc_connections = Arc::clone(&self.connections);
        let is_shutting_down = Arc::clone(&self.shutdown);
        
        thread::spawn(move || {

            let mut connections = arc_connections.lock().unwrap();

            

            while !is_shutting_down.lock().unwrap().clone()
            {

                let mut index = 0;

                while index < connections.len() {
                    let res = connections[index].listen();

                    if res.is_err() {
                        connections.remove(index);
                    }
                    else {
                        index+=1;
                    }
                }

                thread::sleep(Duration::from_millis(SLEEP_TIME));
            }

        })
    }

    pub fn shutdown(&mut self){
        *self.shutdown.lock().unwrap() = true;
    }

}