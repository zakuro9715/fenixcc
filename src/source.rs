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
}

#[test]
fn test_source_eq() {
    let f1 = Source::new("f", "code");
    let f2 = Source::new("f", "code");
    let f3 = Source::new("f", "code2");
    let f4 = Source::new("f1", "code");
    assert_eq!(f1, f2);
    assert_eq!(&f1, &f2);
    assert_ne!(f1, f3);
    assert_ne!(&f1, &f3);
    assert_ne!(f1, f4);
    assert_ne!(&f1, &f4);
}
