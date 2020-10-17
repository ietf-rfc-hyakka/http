mod request_line;

use crate::{HTTPVersion, Method, URI};
use request_line::{parse_request_line, RequestLine};

pub fn parse_request(text: impl AsRef<str>) -> Request {
    let text = text.as_ref();
    let lines = text.split("\r\n");
    let mut lines_iter = lines.into_iter();
    let request_line = {
        let request_line = lines_iter.next().expect("Failed to get Status-Line");
        parse_request_line(request_line)
    };
    todo!()
}

pub struct Request {
    method: Method,
    uri: URI,
    http_version: HTTPVersion,
    content_length: u64,
    user_agent: String,
    headers: Headers,
    body: Body,
}

pub struct Headers;

pub struct Body;
