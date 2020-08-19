use std::fmt::Display;

use crate::string_pointer::{StringPointer, StringPointerError};

pub trait RegexType: Display {
    fn matches_string(&self, string_pointer: &mut StringPointer) -> Result<bool, StringPointerError>;
}