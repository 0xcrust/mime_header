pub mod header;

#[cfg(test)]
pub mod header_tests {
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

        for (header, result) in objects.into_iter() {
            let mut header = header.as_bytes().to_owned();
            assert_eq!(normalize_header_key(header.as_mut()), None);
        }
    }

    #[test]
    pub fn test_mime_header() {
        let mut header = MIMEHeader::new_from("Subject".to_string(), vec!["headerTest".to_string()]);
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
