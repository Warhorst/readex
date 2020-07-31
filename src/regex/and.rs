use crate::regex::Regex;
use crate::regex::regex_type::RegexType;

pub struct And<'a> {
    left: Regex<'a>,
    right: Regex<'a>,
}

impl<'a> RegexType for And<'a> {
    fn matches_string(&self, string: &str) -> bool {
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