use std::net::{SocketAddr, TcpListener};
use crate::http_server::types::{HTTPMethod, Handler};
use std::sync::{Arc, Mutex};
use std::thread;
use crate::http_server::endpoint::EndPointPool;
pub struct HTTPServer {
    socket: SocketAddr,
    endpoint_pool: Arc<Mutex<EndPointPool>>,
}
impl HTTPServer {

    pub fn new(socket_address: &str) -> HTTPServer {
        HTTPServer {
            socket: socket_address.parse().unwrap(),
            endpoint_pool: Arc::new(Mutex::new(EndPointPool::new()))
        }
    }

    pub fn run(&self){
        let listener = TcpListener::bind(self.socket).unwrap();

        println!("Running a HTTPServer on {}", self.socket);

        for socket in listener.incoming() {
            match socket {
                Ok(socket) => {
                        let pool = Arc::clone(&self.endpoint_pool);
                        thread::spawn(move || {
                            pool.lock().unwrap().handle_client(socket);
                        });
                    },
                    _ => {
                        println!("Error occured while processing socket: {:?}", socket);
                        continue;
                    }
                }
        }
    }

    pub fn register_endpoint(&mut self, path: &str, method: HTTPMethod, handler: Handler){
        self.endpoint_pool.lock().unwrap().register_endpoint(path, method, handler);
    }
}

