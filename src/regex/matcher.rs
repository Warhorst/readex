use crate::matcher::Matcher;
use crate::regex::regex_type::RegexType;
use crate::string_pointer::StringPointer;

pub struct Match<M: Matcher> {
    matcher: M
}

impl<M: Matcher> RegexType for Match<M> {
    fn matches_string(&self, string_pointer: &mut StringPointer) -> bool {
        let checked_string_length = self.matcher.checked_string_length();
        match string_pointer.take_next(checked_string_length) {
            Ok(string) => self.matcher.matches(string),
            Err(_) => false
        }
    }
}

impl<M: Matcher> Match<M> {
    pub fn new(matcher: M) -> Self {
        Match {
            matcher
        }
    }
}