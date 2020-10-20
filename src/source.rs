#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Source {
    pub filename: String,
    pub code: Vec<char>,
}

impl Source {
    pub fn new(filename: impl Into<String>, code: impl Into<String>) -> Self {
        Self {
            filename: filename.into(),
            code: code.into().chars().collect(),
        }
    }

    pub fn inline(code: impl Into<String>) -> Self {
        Self::new("__inline__", code)
    }
}

#[test]
fn test_source_new() {
    let emoji = "\u{1f408}";
    let f1 = Source::new("f", emoji);
    let f2 = Source::inline("code");
    assert_eq!(f1.filename, "f".to_string());
    assert_eq!(f1.code.len(), 1);
    assert_eq!(f1.code[0].to_string().len(), 4);
    assert_eq!(f2.code.len(), 4);
}
