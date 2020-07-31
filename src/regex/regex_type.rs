pub trait RegexType {
    fn matches_string(&self, string: &str) -> bool;
}