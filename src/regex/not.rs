use crate::regex::Regex;
use crate::regex::regex_type::RegexType;
use crate::string_pointer::StringPointer;

pub struct Not<'a> {
    inner: Regex<'a>
}

impl<'a> RegexType for Not<'a> {
    fn matches_string(&self, string_pointer: &mut StringPointer) -> bool {
        false
    }
}

impl<'a> Not<'a> {
    pub fn new(inner: Regex<'a>) -> Self {
        Not {
            inner
        }
    }
}