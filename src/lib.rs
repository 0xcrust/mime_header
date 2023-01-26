#![allow(dead_code)]
pub mod header;
pub mod normalize;

#[cfg(test)]
pub mod header_tests {
    use crate::header::*;

    #[test]
    pub fn test_mime_header() {
        let mut header =
            MIMEHeader::new_from("Subject".to_string(), vec!["headerTest".to_string()]);
        assert_eq!(header.get("Subject".to_string()), Some("headerTest"));
        assert_eq!(header.get("User-Agent".to_string()), None);

        header.add("user-aGENt", "mozilla".to_string());
        header.add("Set-Cookie", "xxxx".to_string());
        assert_eq!(header.get("user-aGENT".to_string()), Some("mozilla"));
        assert_eq!(header.get("Set-Cookie".to_string()), Some("xxxx"));

        header.add("Randomheader", "x".to_string());
        assert_eq!(header.get("Randomheader".to_string()), Some("x"));

        header.add("USeR-AGent", "chrome".to_string());
        assert_eq!(
            header.get_all("User-Agent".to_string()),
            Some(&["mozilla".to_string(), "chrome".to_string()][..])
        );
    }
}
