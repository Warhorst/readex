use std::fmt::{Display, Formatter};

use crate::regex::Regex;
use crate::regex::regex_type::RegexType;
use crate::string_pointer::{StringPointer, StringPointerError};

pub struct Not<'a> {
    inner: Regex<'a>
}

impl<'a> RegexType for Not<'a> {
    fn matches_string(&self, string_pointer: &mut StringPointer) -> Result<bool, StringPointerError> {
        Ok(false)
    }
}

impl<'a> Not<'a> {
    pub fn new(inner: Regex<'a>) -> Self {
        Not {
            inner
        }
    }
}

impl<'a> Display for Not<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Not")
    }
}