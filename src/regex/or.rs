use crate::regex::Regex;
use crate::regex::regex_type::RegexType;

pub struct Or<'a> {
    left: Regex<'a>,
    right: Regex<'a>,
}

impl<'a> RegexType for Or<'a> {
    fn matches_string(&self, string: &str) -> bool {
        false
    }
}

impl<'a> Or<'a> {
    pub fn new(left: Regex<'a>, right: Regex<'a>) -> Self {
        Or {
            left,
            right,
        }
    }
}