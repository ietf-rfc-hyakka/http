use tokio::sync::mpsc;

mod http_version;
mod method;
mod request;
mod uri;

pub use crate::{
    http_version::{parse_http_version, HTTPVersion},
    method::{parse_method, Method},
    request::{parse_request, Request},
    uri::{parse_uri, URI},
};

#[tokio::main]
async fn main() {
    env_logger::init();

    let (s_tx, mut s_rx) = mpsc::unbounded_channel();

    tokio::spawn(async move {
        let empty_body = "GET http://localhost/index.html HTTP/1.0 \r\n\
                          User-Agent: CERN-LineMode/2.15 libwww/2.17b3\r\n\
                          Content-Length: 0\r\n\r\n";
        s_tx.send(empty_body)
            .expect("Failed to send first request.");
    });

    while let Some(req) = s_rx.recv().await {
        log::debug!("{:?}", req);
        parse_request(req);
    }
}
