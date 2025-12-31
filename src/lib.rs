#[path ="SocketServer/socket_server_modules/mod.rs"]
pub mod socket_server_modules;

#[path = "HttpServer/mod.rs"]
pub mod http_server;

pub use crate::socket_server_modules::Server as SocketServer;
pub use crate::http_server::Server as HTTPServer;