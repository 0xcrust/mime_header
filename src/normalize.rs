use std::collections::HashMap;

pub fn normalize_header_key(key: &mut [u8]) -> Option<String> {
    let mut no_canon = false;
    for c in key.iter() {
        if is_valid_header_byte(c) {
            continue;
        }
        if *c == b' ' {
            no_canon = true;
            continue;
        }
        return None;
    }
    if no_canon {
        return Some(String::from_utf8(key.into()).unwrap());
    }
    let mut upper = true;

    for c in key.iter_mut() {
        if upper && c.is_ascii_lowercase() {
            *c = c.to_ascii_uppercase();
        } else if !upper && c.is_ascii_uppercase() {
            *c = c.to_ascii_lowercase();
        }
        // We only capitalize the first letter of the
        // header and subsequent letters that follow a hyphen(-)
        upper = *c == b'-'
    }
    let common_headers = common_headers();
    let header = String::from_utf8(key.into()).unwrap();
    if let Some(x) = common_headers.get(&header) {
        return Some(x.clone());
    }
    Some(header)
}

fn is_valid_header_byte(c: &u8) -> bool {
    let symbols: Vec<char> = vec![
        '!', '#', '$', '%', '&', '\'', '*', '+', '-', '.', '^', '_', '`', '|', '~',
    ];
    let mut range = vec!['0'..='9', 'a'..='z', 'A'..='Z'];

    let range = range
        .iter_mut()
        .flatten()
        .collect::<Vec<char>>()
        .into_iter()
        .chain(symbols.into_iter())
        .any(|x| x == (*c as char));

    range
}

fn common_headers() -> HashMap<String, String> {
    let common_headers = vec![
        "Accept",
        "Accept-Charset",
        "Accept-Encoding",
        "Accept-Language",
        "Accept-Ranges",
        "Cache-Control",
        "Cc",
        "Connection",
        "Content-Id",
        "Content-Language",
        "Content-Length",
        "Content-Transfer-Encoding",
        "Content-Type",
        "Cookie",
        "Date",
        "Dkim-Signature",
        "Etag",
        "Expires",
        "From",
        "Host",
        "If-Modified-Since",
        "If-None-Match",
        "In-Reply-To",
        "Last-Modified",
        "Location",
        "Message-Id",
        "Mime-Version",
        "Pragma",
        "Received",
        "Return-Path",
        "Server",
        "Set-Cookie",
        "Subject",
        "To",
        "User-Agent",
        "Via",
        "X-Forwarded-For",
        "X-Imforwards",
        "X-Powered-By",
    ];

    common_headers
        .into_iter()
        .map(|x| (x.to_string(), x.to_string()))
        .collect()
}

#[cfg(test)]
pub mod normalize_tests {
    use super::*;

    #[test]
    pub fn test_valid_header_byte() {
        let objects = vec![
            'a', 'b', '1', '2', 'r', 'z', 'g', '+', '-', '_', '^', '$', '3', '4', '1', '1', 'f',
            'A', 'Z', 'X',
        ];
        for i in objects.into_iter() {
            assert_eq!(is_valid_header_byte(&(i as u8)), true);
        }
    }

    #[test]
    pub fn test_invalid_header_byte() {
        let objects = vec!['@', '{', '}', ' ', '=', ']', '[', '(', ')'];
        for i in objects.into_iter() {
            assert_eq!(is_valid_header_byte(&(i as u8)), false);
        }
    }

    #[test]
    pub fn test_normalize_valid() {
        let objects = vec![
            ("a-b-c", "A-B-C"),
            ("a-1-c", "A-1-C"),
            ("User-Agent", "User-Agent"),
            ("uSER-aGENT", "User-Agent"),
            ("user-agent", "User-Agent"),
            ("USER-AGENT", "User-Agent"),
            ("foo-bar_baz", "Foo-Bar_baz"),
            ("foo-bar$baz", "Foo-Bar$baz"),
            ("foo-bar~baz", "Foo-Bar~baz"),
            ("foo-bar*baz", "Foo-Bar*baz"),
        ];

        for (header, result) in objects.into_iter() {
            let mut header = header.as_bytes().to_owned();
            assert_eq!(
                normalize_header_key(header.as_mut()),
                Some(result.to_string())
            );
        }
    }

    #[test]
    pub fn test_valid_not_normalized() {
        let objects = vec![
            ("a B", "a B"),
            ("U ser-A gent", "U ser-A gent"),
            ("Re ce ived", "Re ce ived"),
            ("C Ontent-Transfer-Encoding", "C Ontent-Transfer-Encoding"),
            ("foo bar", "foo bar"),
        ];

        for (header, result) in objects.into_iter() {
            let mut header = header.as_bytes().to_owned();
            assert_eq!(
                normalize_header_key(header.as_mut()),
                Some(result.to_string())
            );
        }
    }

    #[test]
    pub fn test_invalid_not_normalized() {
        let objects = vec![
            ("üser-agenT", "üser-agenT"),
            ("こんにちは", "こんにちは"),
            ("Здравствуйте", "Здравствуйте"),
            ("Dobrý den", "Dobrý den"),
            ("Olá", "Olá"),
        ];

        for (header, _) in objects.into_iter() {
            let mut header = header.as_bytes().to_owned();
            assert_eq!(normalize_header_key(header.as_mut()), None);
        }
    }
}
