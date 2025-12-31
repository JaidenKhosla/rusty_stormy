
use crate::http_server::{types::{HTTPMethod, HTTPRequest, Handler, Response, StringMap}};
use std::{collections::HashMap, io::{Read, Write}};
use std::net::{TcpStream};
use regex::Regex;

#[path ="response.rs"]
mod response;

const MAX_HTTP_REQ_LENGTH: usize = 8192;
pub struct EndPoint {
    pub path: String,
    pub handler: Handler
}
impl EndPoint {
    fn new(path: &str, handler: Handler) -> EndPoint {
        EndPoint {
            path: path.to_string(),
            handler: handler
        }
    } 
}
pub struct EndPointPool {
    pub _endpoints: HashMap<String, EndPoint>,
    endpoint_404: Handler,
    pattern: Regex
}

impl EndPointPool{
    pub fn new() -> EndPointPool {
        EndPointPool { _endpoints: HashMap::new(), pattern: Regex::new(r"(\\0)+$").unwrap(), endpoint_404: default_404 }
    }

    #[allow(dead_code)]
    fn set404(&mut self, handler: Handler){
        self.endpoint_404 = handler;
    }

    fn process_request(&self, socket: &mut TcpStream) -> HTTPRequest
    {
        let mut buff = [0u8; MAX_HTTP_REQ_LENGTH];

        let _ = socket.read(&mut buff);

        let parsed_string = String::from_utf8_lossy(&buff);

        let header_and_body: Vec<&str> = parsed_string.split("\r\n\r\n").collect();

        let mut header_map: StringMap = HashMap::new();

        let headers = header_and_body[0].split("\r\n").collect::<Vec<&str>>();

        for i in 1..headers.len() {
            let key_value = &headers[i].split(": ").collect::<Vec<&str>>();
            let key = (&key_value[0]).to_string();
            let value = (&key_value[1]).to_string();
            header_map.insert(key, value);
        }

        let signature = headers[0].split(" ").collect::<Vec<&str>>();

        let method = HTTPMethod::from(signature[0]).unwrap();
        let path = signature[1];

        let request = HTTPRequest {
            method: method,
            path: path.to_string(),
            headers: header_map,
            body: self.pattern.replace_all(header_and_body[1], "").to_string()
        };

        return request;

    }  

    pub fn register_endpoint(&mut self, path: &str, method: HTTPMethod, handler: Handler){
        let endpoint = EndPoint::new(path, handler);
        
        let key = format!("{} {}", endpoint.path, method);

        self._endpoints.insert(key, endpoint);
    }

    pub fn handle_client(&mut self, mut socket: TcpStream){
        let parsed_req = self.process_request(&mut socket);
        println!("{:?}", parsed_req);
        let endpoint = self._endpoints.get(&format!("{} {}", parsed_req.path, parsed_req.method));

        if endpoint.is_none()
        {
            let _ = (self.endpoint_404)(&parsed_req);
        }
        else {
            let res = (endpoint.unwrap().handler)(&parsed_req);

            if res.is_err() {
                println!("{}", res.unwrap_err());
            }
            else {
                let ( status_code, content_type, body ) = res.unwrap();

                let response = response::build_response(status_code, &parsed_req.headers, &content_type, &body);
            
                let _ = socket.write_all(response.as_bytes());
            }
        }
    }

}

fn default_404<'a>(_req: &HTTPRequest) -> Response<'a>{
    return Ok((404, "text/plain", ""));
}  
