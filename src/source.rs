#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Source {
    pub filename: String,
    pub code: Vec<char>,
}

impl Source {
    pub fn new(filename: String, code: String) -> Self {
        Self {
            filename,
            code: code.chars().collect(),
        }
    }
}

#[test]
fn test_source_eq() {
    let f1 = Source::new("f".to_string(), "code".to_string());
    let f2 = Source::new("f".to_string(), "code".to_string());
    let f3 = Source::new("f".to_string(), "code2".to_string());
    let f4 = Source::new("f1".to_string(), "code".to_string());
    assert_eq!(f1, f2);
    assert_eq!(&f1, &f2);
    assert_ne!(f1, f3);
    assert_ne!(&f1, &f3);
    assert_ne!(f1, f4);
    assert_ne!(&f1, &f4);
}
