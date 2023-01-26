use crate::normalize::normalize_header_key;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct MIMEHeader(HashMap<String, Vec<String>>);

impl MIMEHeader {
    pub fn new() -> MIMEHeader {
        MIMEHeader(HashMap::new())
    }

    pub fn new_from(key: String, values: Vec<String>) -> MIMEHeader {
        let mut header = Self::new();
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
