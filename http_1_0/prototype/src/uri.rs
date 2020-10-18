pub fn parse_uri(uri_str: impl AsRef<str>) -> URI {
    let uri_str = uri_str.as_ref();
    if uri_str.starts_with("http://") {
        let content = &uri_str["http://".len()..];
        let mut content_iter = content.split("/").into_iter();

        let authority = parse_authority(content_iter.next().expect("Failed to unwrap authority"));
        let path = parse_path(content_iter.next().expect("Failed to unwrap path"));

        todo!()
    } else {
        todo!()
    }
}

fn parse_authority(target: impl AsRef<str>) -> Authority {
  let mut content = target.as_ref().split(":").into_iter();
  let host = content.next().expect("Failed to unwrap host").to_string();
  let port = content.next().expect("Failed to unwrap port").to_string();
  Authority { host, port }
}

fn parse_path(target: impl AsRef<str>) -> Path {
  let mut content = target.as_ref().split("?").into_iter();
  let path = content.next().expect("Failed to unwrap host").to_string();
  let searchpart = content.next().expect("Failed to unwrap port").to_string();
  Path { path, searchpart }
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

struct Authority {
  host: String,
  port: String,
}

struct Path {
  path: String,
  searchpart: String,
}

struct Location {
    host: String,
    port: Option<u64>,
}
