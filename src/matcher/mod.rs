use crate::string_pointer::StringPointer;

pub trait Matcher {
    fn matches(&self, string_pointer: StringPointer) -> bool;
}

pub struct StupidMatcher;

pub mod string_matcher;

impl Matcher for StupidMatcher {
    fn matches(&self, _string_pointer: StringPointer) -> bool {
        false
    }
}