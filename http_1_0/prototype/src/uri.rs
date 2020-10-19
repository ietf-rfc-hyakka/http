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

fn get_head_tail(content: impl AsRef<str>, pipe: impl AsRef<str>) -> HeadTail {
  let mut iter = content.as_ref().split(pipe.as_ref()).into_iter();
  let head = iter.next().expect("Failed to unwrap head").to_string();
  // port が指定されていない, query が指定されていないケースに備えて match で処理している
  let tail = match iter.next() {
    Some(data) => data.to_string(),
    None => String::new()
  };
  HeadTail { head, tail }
}

fn parse_authority(target: impl AsRef<str>) -> Authority {
  let content = get_head_tail(target.as_ref(), ":");
  let (host, port) = (content.head, content.tail);
  Authority { host, port }
}

fn parse_path(target: impl AsRef<str>) -> Path {
  let content = get_head_tail(target.as_ref(), "?");
  let (path, searchpart) = (content.head, content.tail);
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

struct HeadTail {
  head: String,
  tail: String,
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
