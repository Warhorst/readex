use crate::regex::Regex;
use crate::regex::regex_type::RegexType;
use crate::string_pointer::StringPointer;

pub struct And<'a> {
    left: Regex<'a>,
    right: Regex<'a>,
}

impl<'a> RegexType for And<'a> {
    fn matches_string(&self, string_pointer: &mut StringPointer) -> bool {
        false
    }
}

impl<'a> And<'a> {
    pub fn new(left: Regex<'a>, right: Regex<'a>) -> Self {
        And {
            left,
            right,
        }
    }
}