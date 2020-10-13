use tokio::sync::mpsc;

struct Request {
    method: Method,
    uri: URI,
    http_version: HTTPVersionKind,
    content_length: u64,
    user_agent: String,
    headers: Headers,
    body: Body,
}

enum Method {
    GET,
    HEAD,
    POST,
    Other(String),
}

enum HTTPVersionKind {
    V1_0,
}

enum URI {
    AbsoluteURI(AbsoluteURI),
    AbsolutePath(std::path::PathBuf),
}

struct AbsoluteURI {
    scheme: Scheme,
    name: Option<String>,
    password: Option<String>,
    host: String,
    port: Option<u64>,
    url_path: std::path::PathBuf,
}

enum Scheme {
    HTTP,
    Other(String),
}

struct Headers;
struct Body;

#[tokio::main]
async fn main() {
    env_logger::init();

    let (ua_tx, mut ua_rx) = mpsc::unbounded_channel();
    let (s_tx, mut s_rx) = mpsc::unbounded_channel();

    tokio::spawn(async move {
        let empty_body = "GET http://localhost/index.html HTTP/1.0 \r\n\
                          User-Agent: CERN-LineMode/2.15 libwww/2.17b3\r\n\
                          Content-Length: 0\r\n\r\n";
        s_tx.send(empty_body).unwrap();
    });

    while let Some(req) = s_rx.recv().await {
        log::debug!("{:?}", req);
        parse(req);
    }
}

fn parse(text: impl Into<String>) -> Request {
    let text = text.into();
    let lines = text.split("\r\n");
    let mut lines_iter = lines.into_iter();
    let _ = {
        let request_line = lines_iter.next().unwrap();
        let items = text.split_ascii_whitespace();
        let mut items_iter = items.into_iter();
        let method = {
            let method = items_iter.next().unwrap();
            match method {
                "GET" => Method::GET,
                "HEAD" => Method::HEAD,
                "POST" => Method::POST,
                other => Method::Other(other.to_string()),
            }
        };
        let uri = {
            let uri = items_iter.next().unwrap();
            if uri.starts_with("http://") {
                let content = &uri["http://".len()..];
                let content_iter = content.split("@").into_iter();
                let (authority, location) = (content_iter.next(), content_iter.next());
            } else {
            }
        };
    };
    todo!()
}
