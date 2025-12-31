use serde_json::{Value, from_str};
use std::{sync::{OnceLock}};

use crate::http_server::types::*;
#[allow(dead_code)]
static mut JSON_CODES: &'static mut OnceLock<Value> = &mut OnceLock::new();

#[allow(dead_code)]
const HTTP_VERSION: &str = "1.1";

#[allow(dead_code)]
pub fn build_response(
    status_code: i32,
    headers: &StringMap,
    content_type: &str,
    body: &str
) -> String {

    let map = unsafe {
        (*&raw const JSON_CODES).get_or_init(|| from_str(include_str!("./codes.json")).unwrap())
    };

    let mut header_string = String::new();

    headers.keys().for_each(|k| header_string.push_str(format!("{}: {}\n", k, headers[k]).as_str()));

    format!(r#"
HTTP/{version} {status_code} {reason}
{headers}
Content-Type: {content_type}
Content-Length: {length}

{body}"#, version=HTTP_VERSION, status_code=status_code, reason=map[status_code.to_string()], headers=header_string, content_type=content_type, length=body.as_bytes().len())
}