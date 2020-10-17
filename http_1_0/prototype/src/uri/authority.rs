fn parse_uri_authority(authority_str: impl AsRef<str>) -> Authority {
    let authority_str = authority_str.as_ref();
    if authority_str.is_empty() {
        return Authority::Nothing;
    };

    let mut authority_iter = authority_str.split(":").into_iter();
    // 上で空の文字列かどうか試しているので、ここでは必ず文字列が入る。
    // よってunwrap()を使う。
    let name = authority_iter.next().unwrap();
    if let Some(password) = authority_iter.next() {
        Authority::NameWithPassword(name.to_string(), password.to_string())
    } else {
        Authority::NameOnly(name.to_string())
    }
}

#[derive(Debug, PartialEq)]
pub enum Authority {
    Nothing,
    NameOnly(String),
    NameWithPassword(String, String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let empty_str = "";
        let expect = Authority::Nothing;

        let actual = parse_uri_authority(empty_str);

        assert_eq!(actual, expect);
    }

    #[test]
    fn name_only() {
        let name_str = "name";
        let expect = Authority::NameOnly(name_str.to_string());

        let actual = parse_uri_authority(name_str);

        assert_eq!(actual, expect);
    }

    #[test]
    fn name_and_password() {
        let name_str = "name";
        let password_str = "";
        let input_str = format!("{}:{}", name_str, password_str);
        let expect = Authority::NameWithPassword(name_str.to_string(), password_str.to_string());

        let actual = parse_uri_authority(input_str);

        assert_eq!(actual, expect);
    }
}