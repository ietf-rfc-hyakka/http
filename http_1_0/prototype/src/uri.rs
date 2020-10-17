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

fn parse_uri_authority(authority_str: impl AsRef<str>) -> Authority {
    let authority_str = authority_str.as_ref();
    let mut authority_iter = authority_str.split(":").into_iter();
    let (name, password) = (
        authority_iter.next().map(ToString::to_string),
        authority_iter.next().map(ToString::to_string),
    );
    Authority { name, password }
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
    name: Option<String>,
    password: Option<String>,
}

struct Location {
    host: String,
    port: Option<u64>,
}
