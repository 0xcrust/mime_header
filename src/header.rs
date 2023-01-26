use std::{collections::HashMap};

#[derive(Debug)]
struct MIMEHeader(HashMap<String, Vec<String>>);

impl MIMEHeader {
    pub fn new() -> MIMEHeader {
        MIMEHeader(HashMap::new())
    }

    pub fn new_from(key: String, values: Vec<String>) -> MIMEHeader {
        let mut header =  Self::new();
        for value in values.into_iter() {
            header.add(&key, value);
        }
        header
    }

    pub fn add(&mut self, key: &str, value: String) {
        let key = Self::normalize_header_key(key.to_string());
        if let Some(values) = self.0.get_mut(&key) {
            values.push(value);
        } else {
            self.0.insert(key, vec![value]);
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        let key = Self::normalize_header_key(key);
        self.0.insert(key, vec![value]);
    }

    pub fn get(&self, key: String) -> Option<&str> {
        let key = Self::normalize_header_key(key);
        match self.0.get(&key) {
            Some(values) => Some(&values[0]),
            None => None,
        }
    }

    pub fn get_all(&self, key: String) -> Option<&[String]> {
        let key = Self::normalize_header_key(key);
        match self.0.get(&key) {
            Some(values) => Some(&values[..]),
            None => None,
        }
    }

    pub fn delete(&mut self, key: String) -> Option<Vec<String>> {
        let key = Self::normalize_header_key(key);
        self.0.remove(&key)
    }

    fn normalize_header_key(key: String) -> String {
        let mut key = key.as_bytes().to_owned();
        match normalize_header_key(key.as_mut()) {
            Some(normalized) => normalized,
            None => String::from_utf8(key).unwrap(),
        }
    }
}

fn normalize_header_key(key: &mut [u8]) -> Option<String> {
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
