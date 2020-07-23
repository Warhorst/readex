pub trait Matcher {
    fn matches(&self, string: &str) -> bool;
}

pub struct StupidMatcher;

impl Matcher for StupidMatcher {
    fn matches(&self, string: &str) -> bool {
        false
    }
}