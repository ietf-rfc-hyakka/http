use crate::{parse_http_version, parse_method, parse_uri, HTTPVersion, Method, URI};

pub struct RequestLine {
    method: Method,
    uri: URI,
    version: HTTPVersion,
}

pub fn parse_request_line(request_line: impl AsRef<str>) -> RequestLine {
    let request_line = request_line.as_ref();
    let status = request_line.split_ascii_whitespace();
    let mut status_iter = status.into_iter();
    let method = {
        let method_str = status_iter.next().expect("Failed to get HTTP Method");
        parse_method(method_str)
    };
    let uri = {
        let uri_str = status_iter.next().expect("Failed to get URI");
        parse_uri(uri_str)
    };
    let http_version = {
        let http_version_str =
            &status_iter.next().expect("Failed to get HTTP Version")["HTTP/".len()..];
        parse_http_version(http_version_str);
    };
    todo!()
}
