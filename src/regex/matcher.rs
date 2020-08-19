use std::fmt::{Display, Formatter};

use crate::matcher::Matcher;
use crate::regex::regex_type::RegexType;
use crate::string_pointer::{StringPointer, StringPointerError};

pub struct Match<M: Matcher> {
    matcher: M
}

impl<M: Matcher> RegexType for Match<M> {
    fn matches_string(&self, string_pointer: &mut StringPointer) -> Result<bool, StringPointerError> {
        let checked_string_length = self.matcher.checked_string_length();
        match string_pointer.take_next(checked_string_length) {
            Ok(string) => Ok(self.matcher.matches(string)),
            Err(error) => Err(error)
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

impl<M: Matcher> Display for Match<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Match")
    }
}