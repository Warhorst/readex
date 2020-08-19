use std::fmt::{Display, Formatter};

use crate::regex::Regex;
use crate::regex::regex_type::RegexType;
use crate::string_pointer::{StringPointer, StringPointerError};

pub struct Or<'a> {
    left: Regex<'a>,
    right: Regex<'a>,
}

impl<'a> RegexType for Or<'a> {
    fn matches_string(&self, string_pointer: &mut StringPointer) -> Result<bool, StringPointerError> {
        Ok(false)
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

impl<'a> Display for Or<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Or")
    }
}