pub fn parse_method(method_str: impl AsRef<str>) -> Method {
    match method_str.as_ref() {
        "GET" => Method::GET,
        "HEAD" => Method::HEAD,
        "POST" => Method::POST,
        other => Method::Other(other.to_string()),
    }
}

pub enum Method {
    GET,
    HEAD,
    POST,
    Other(String),
}
