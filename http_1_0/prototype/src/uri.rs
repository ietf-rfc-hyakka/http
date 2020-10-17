mod authority;

use authority::Authority;

pub fn parse_uri(uri_str: impl AsRef<str>) -> URI {
    let uri_str = uri_str.as_ref();
    if uri_str.starts_with("http://") {
        let content = &uri_str["http://".len()..];
        let content_iter = content.split("@").into_iter();
        todo!()
    } else {
        todo!()
    }
}

pub enum URI {
    AbsoluteURI(AbsoluteURI),
    AbsolutePath(std::path::PathBuf),
}

pub struct AbsoluteURI {
    scheme: Scheme,
    authority: Authority,
    location: Location,
    url_path: std::path::PathBuf,
}

enum Scheme {
    HTTP,
    Other(String),
}


struct Location {
    host: String,
    port: Option<u64>,
}
