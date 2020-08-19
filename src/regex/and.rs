use std::fmt::{Display, Formatter};

use crate::regex::Regex;
use crate::regex::regex_type::RegexType;
use crate::string_pointer::{StringPointer, StringPointerError};

pub struct And<'a> {
    left: Regex<'a>,
    right: Regex<'a>,
}

impl<'a> RegexType for And<'a> {
    fn matches_string(&self, string_pointer: &mut StringPointer) -> Result<bool, StringPointerError> {
        Ok(false)
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

impl<'a> Display for And<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "And")
    }
}