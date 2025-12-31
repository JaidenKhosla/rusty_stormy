
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub enum HTTPMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    OPTIONS,
    COPY,
    LINK,
    UNLINK,
    PURGE
}

impl HTTPMethod {
    pub fn from(from: &str) -> Result<HTTPMethod, &str> {
        return match from {
            "GET" => Ok(HTTPMethod::GET),
            "POST" => Ok(HTTPMethod::POST),
            "PUT" => Ok(HTTPMethod::PUT),
            "PATCH" => Ok(HTTPMethod::PATCH),
            "DELETE" => Ok(HTTPMethod::DELETE),
            "HEAD" => Ok(HTTPMethod::HEAD),
            "OPTIONS" => Ok(HTTPMethod::OPTIONS),
            "COPY" => Ok(HTTPMethod::COPY),
            "LINK" => Ok(HTTPMethod::LINK),
            "UNLINK" => Ok(HTTPMethod::UNLINK),
            "PURGE" => Ok(HTTPMethod::PURGE),
            _ => Err("Invalid str. Can't be converted to an HTTPMethod enum")
        }
    }
}

impl fmt::Display for HTTPMethod {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct HTTPRequest {
    pub path: String,
    pub method: HTTPMethod,
    pub headers: StringMap,
    pub body: String
}

impl HTTPRequest {
}

pub type Response<'a> = Result<(i32, &'a str, &'a str), &'a str>;

pub type Handler = fn(req: &HTTPRequest) -> Response<'_>;
pub type StringMap = HashMap<String, String>;

